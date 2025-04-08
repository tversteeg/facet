use crate::parser::{JsonParseErrorKind, JsonParseErrorWithContext, JsonParser};

use facet_poke::Poke;
use facet_trait::{Facet, Opaque, OpaqueConst, OpaqueUninit, ShapeExt as _};
use log::trace;

/// Deserializes a JSON string into a value of type `T` that implements `Facet`.
///
/// This function takes a JSON string representation and converts it into a Rust
/// value of the specified type `T`. The type must implement the `Facet` trait
/// to provide the necessary type information for deserialization.
///
/// # Parameters
/// * `json` - A string slice containing the JSON to deserialize
///
/// # Returns
/// * `Ok(T)` - The successfully deserialized value
/// * `Err(JsonParseErrorWithContext)` - An error with context if deserialization fails
///
/// # Example
/// ```
/// # use facet_trait::Facet, ;
/// # use facet_derive::Facet;
/// # use facet_trait as facet;
/// # #[derive(Facet)]
/// # struct Person { name: String, age: u64 }
/// let json = r#"{"name":"Alice","age":30}"#;
/// let person: Person = facet_json::from_str(json).unwrap();
/// ```
pub fn from_str<T: Facet>(json: &str) -> Result<T, JsonParseErrorWithContext<'_>> {
    let (poke, _guard) = Poke::alloc::<T>();
    let opaque = from_str_opaque(poke, json)?;
    Ok(unsafe { opaque.read::<T>() })
}

/// Deserialize a `Poke` object from a JSON string.
pub fn from_str_opaque<'input, 'mem>(
    poke: Poke<'mem>,
    json: &'input str,
) -> Result<Opaque<'mem>, JsonParseErrorWithContext<'input>> {
    trace!("Starting JSON deserialization");
    let mut parser = JsonParser::new(json);
    deserialize_value(&mut parser, poke)
}

/// Deserializes a value from JSON using an iterative approach.
///
/// This function takes a JSON parser and a Poke object and deserializes the JSON
/// into the Poke object. It uses an iterative approach with a stack to avoid
/// recursion.
fn deserialize_value<'input, 'mem>(
    parser: &mut JsonParser<'input>,
    root_poke: Poke<'mem>,
) -> Result<Opaque<'mem>, JsonParseErrorWithContext<'input>> {
    use std::collections::VecDeque;

    // Define various states for the deserialization process
    enum StackItem<'mem> {
        // For processing a value
        Value {
            poke: Poke<'mem>,
        },
        // For finishing a struct after processing its fields
        FinishStruct {
            ps: facet_poke::PokeStruct<'mem>,
        },
        // For processing a struct field
        StructField {
            ps: facet_poke::PokeStruct<'mem>,
            key: String,
        },
        // For handling operations after processing a struct field's value
        AfterStructField {
            ps: facet_poke::PokeStruct<'mem>,
            index: usize,
        },
        // For finishing a list after processing its items
        FinishList {
            pl: facet_poke::PokeList<'mem>,
        },
        // For processing a list item
        ListItem {
            pl: facet_poke::PokeList<'mem>,
            shape: &'static facet_trait::Shape,
            index: usize,
        },
        // For finishing a map after processing its entries
        FinishMap {
            pm: facet_poke::PokeMap<'mem>,
        },
        // For processing a map entry
        MapEntry {
            pm: facet_poke::PokeMap<'mem>,
            key: String,
        },
    }

    // Initialize the result and the stack
    let mut result = None;
    let mut stack = VecDeque::new();
    stack.push_back(StackItem::Value { poke: root_poke });

    // Process the stack until it's empty
    while let Some(item) = stack.pop_front() {
        match item {
            StackItem::Value { poke } => {
                let shape = poke.shape();
                trace!("Deserializing {shape}");

                match poke {
                    Poke::Scalar(pv) => {
                        trace!("Deserializing \x1b[1;36mscalar\x1b[0m");
                        let opaque = if pv.shape().is_type::<String>() {
                            let s = parser.parse_string()?;
                            let data = unsafe { pv.put(OpaqueConst::from_ref(&s)) };
                            std::mem::forget(s);
                            data
                        } else if pv.shape().is_type::<u64>() {
                            let n = parser.parse_u64()?;
                            unsafe { pv.put(OpaqueConst::from_ref(&n)) }
                        } else {
                            panic!("Unknown scalar shape: {}", pv.shape());
                        };
                        result = Some(opaque);
                    }
                    Poke::Struct(ps) => {
                        trace!("Deserializing \x1b[1;36mstruct\x1b[0m");
                        // First, push a FinishStruct item to build the struct in place after processing all fields
                        stack.push_front(StackItem::FinishStruct { ps: ps.clone() });

                        // Then, prepare to process the first field
                        let first_key = parser.expect_object_start()?;
                        if let Some(key) = first_key {
                            // Since we now have Clone, this is much simpler
                            stack.push_front(StackItem::StructField { ps, key });
                        }
                    }
                    Poke::List(list_uninit) => {
                        trace!("Deserializing \x1b[1;36marray\x1b[0m");
                        // Parse array start
                        parser.expect_array_start()?;

                        // Initialize the list with no size hint
                        let pl = list_uninit.init(None).unwrap_or_else(|_| {
                            panic!("Failed to initialize list");
                        });

                        // Prepare to process the first item
                        let has_element = parser.parse_array_element()?;

                        if let Some(true) = has_element {
                            // Store the list in the finish item first - use Clone
                            stack.push_front(StackItem::FinishList { pl: pl.clone() });

                            // Process first item - create a new allocation for it
                            let item_data =
                                OpaqueUninit::new(unsafe { std::alloc::alloc(shape.layout) });
                            let item_poke = unsafe { Poke::unchecked_new(item_data, shape) };

                            // Value needs to be processed next, so put it at the front
                            stack.push_front(StackItem::Value { poke: item_poke });

                            // Then we'll process the ListItem that will handle the result
                            stack.push_front(StackItem::ListItem {
                                pl,
                                shape,
                                index: 0,
                            });
                        } else {
                            // No items, just finish the list
                            stack.push_front(StackItem::FinishList { pl });
                        }
                    }
                    Poke::Map(map_uninit) => {
                        trace!("Deserializing \x1b[1;36mhashmap\x1b[0m");
                        // Parse object start and get first key if it exists
                        let first_key = parser.expect_object_start()?;

                        // Initialize the map with no size hint
                        let initialized_map = map_uninit.init(None).unwrap_or_else(|_| {
                            panic!("Failed to initialize map"); // TODO: map errors
                        });

                        if let Some(key) = first_key {
                            // Now that we have Clone for PokeMap, we can simplify this
                            let value_shape = initialized_map.def.v;

                            // Put our initialized map in the FinishMap entry
                            stack.push_front(StackItem::FinishMap {
                                pm: initialized_map.clone(),
                            });

                            // Create a poke for the value
                            let value_data =
                                OpaqueUninit::new(unsafe { std::alloc::alloc(value_shape.layout) });
                            let value_poke =
                                unsafe { Poke::unchecked_new(value_data, value_shape) };

                            // Value needs to be processed first
                            stack.push_front(StackItem::Value { poke: value_poke });

                            // After processing the value, handle the insertion
                            stack.push_front(StackItem::MapEntry {
                                pm: initialized_map,
                                key,
                            });
                        } else {
                            // No entries, just finish the map
                            stack.push_front(StackItem::FinishMap {
                                pm: initialized_map,
                            });
                        }
                    }
                    Poke::Enum(pe) => {
                        trace!("Deserializing \x1b[1;36menum\x1b[0m");
                        // Assuming enums are serialized as JSON strings representing the variant name
                        let variant_str = parser.parse_string()?;

                        let pe = pe.set_variant_by_name(&variant_str).map_err(|_| {
                            parser.make_error(JsonParseErrorKind::Custom(format!(
                                "Invalid enum variant: {}",
                                variant_str
                            )))
                        })?;

                        trace!("Finished deserializing \x1b[1;36menum\x1b[0m");
                        let opaque = pe.build_in_place();
                        result = Some(opaque);
                    }
                }
            }
            StackItem::StructField { mut ps, key } => {
                trace!("Processing struct key: \x1b[1;33m{}\x1b[0m", key);

                match ps.field_by_name(&key) {
                    Ok((index, field_poke)) => {
                        trace!("Found field, it's at index: \x1b[1;33m{index}\x1b[0m");

                        // After we process the field value, we need to handle post-processing
                        stack.push_front(StackItem::AfterStructField {
                            ps: ps.clone(),
                            index,
                        });

                        // Push the field value to be processed next
                        stack.push_front(StackItem::Value { poke: field_poke });
                    }
                    Err(_) => {
                        trace!("No field named \x1b[1;36m{}\x1b[0m", key);
                        return Err(parser.make_error(JsonParseErrorKind::UnknownField(key)));
                    }
                }
            }
            StackItem::AfterStructField { mut ps, index } => {
                trace!("After processing struct field at index: \x1b[1;33m{index}\x1b[0m");

                // Mark the field as initialized now that we've processed its value
                unsafe { ps.mark_initialized(index) };

                // Now it's the correct time to parse the next key, if there is one
                let next_key = parser.parse_object_key()?;
                if let Some(next_key) = next_key {
                    stack.push_front(StackItem::StructField {
                        ps: ps.clone(),
                        key: next_key,
                    });
                }
            }
            StackItem::FinishStruct { ps } => {
                trace!("Finished deserializing \x1b[1;36mstruct\x1b[0m");
                let opaque = ps.build_in_place();
                result = Some(opaque);
            }
            StackItem::ListItem {
                mut pl,
                shape,
                index,
            } => {
                trace!("Processing array item at index: \x1b[1;33m{}\x1b[0m", index);

                // Allocate memory for the item
                let data = OpaqueUninit::new(unsafe { std::alloc::alloc(shape.layout) });
                let item_poke = unsafe { Poke::unchecked_new(data, shape) };

                // Push the item to be processed next
                stack.push_front(StackItem::Value { poke: item_poke });

                // Check if there's another element after this one
                let has_next = parser.parse_array_element()?;
                if let Some(true) = has_next {
                    // Use Clone to create a copy of the list for the next item
                    stack.push_front(StackItem::ListItem {
                        pl: pl.clone(),
                        shape,
                        index: index + 1,
                    });
                }

                // After processing the item value, add it to the list
                let opaque = result.take().expect("Expected an item result");
                unsafe {
                    pl.push(opaque);
                }
            }
            StackItem::FinishList { pl } => {
                trace!("Finished deserializing \x1b[1;36marray\x1b[0m");
                let opaque = pl.build_in_place();
                result = Some(opaque);
            }
            StackItem::MapEntry { mut pm, key } => {
                trace!("Processing hashmap key: \x1b[1;33m{}\x1b[0m", key);

                // Create a poke for the key (string type)
                let key_data = OpaqueUninit::new(unsafe { std::alloc::alloc(pm.def.k.layout) });
                let key_poke = unsafe { Poke::unchecked_new(key_data, pm.def.k) };
                let scalar_key_poke = key_poke.into_scalar();
                scalar_key_poke.parse(&key).unwrap(); // TODO: map errors

                // Create a poke for the value based on map def
                let value_data = OpaqueUninit::new(unsafe { std::alloc::alloc(pm.def.v.layout) });
                let value_poke = unsafe { Poke::unchecked_new(value_data, pm.def.v) };

                // Push the value to be processed next
                stack.push_front(StackItem::Value { poke: value_poke });

                // Get the next key if there is one
                let next_key = parser.parse_object_key()?;
                if let Some(next_key) = next_key {
                    // We can clone the map for the next entry since Clone is available
                    stack.push_front(StackItem::MapEntry {
                        pm: pm.clone(),
                        key: next_key,
                    });
                }

                // After processing the value, insert the key-value pair into the map
                let value_opaque = result.take().expect("Expected a value result");
                unsafe {
                    pm.insert(key_data.assume_init(), value_opaque);
                }
            }
            StackItem::FinishMap { pm } => {
                trace!("Finished deserializing \x1b[1;36mhashmap\x1b[0m");
                let opaque = pm.build_in_place();
                result = Some(opaque);
            }
        }
    }

    // Return the final result
    result.ok_or_else(|| {
        parser.make_error(JsonParseErrorKind::Custom(
            "Unexpected end of input".to_string(),
        ))
    })
}
