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

    enum StackItem<'mem> {
        Value { poke: Poke<'mem> },
        FinishStruct { ps: facet_poke::PokeStruct<'mem> },
        StructField { key: String },
        AfterStructField { index: usize },
        FinishList { pl: facet_poke::PokeList<'mem> },
        AfterListItem { item: OpaqueUninit<'mem> },
        FinishMap { pm: facet_poke::PokeMap<'mem> },
        AfterMapEntry { key: String },
    }

    let mut result = None;
    let mut stack = VecDeque::new();
    stack.push_back(StackItem::Value { poke: root_poke });

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
                        stack.push_front(StackItem::FinishStruct { ps });

                        let first_key = parser.expect_object_start()?;
                        if let Some(key) = first_key {
                            stack.push_front(StackItem::StructField { key });
                        }
                    }
                    Poke::List(list_uninit) => {
                        trace!("Deserializing \x1b[1;36marray\x1b[0m");
                        parser.expect_array_start()?;

                        let pl = list_uninit.init(None).unwrap_or_else(|_| {
                            panic!("Failed to initialize list");
                        });

                        let has_element = parser.parse_array_element()?;

                        if let Some(true) = has_element {
                            stack.push_front(StackItem::FinishList { pl });

                            let item_data =
                                OpaqueUninit::new(unsafe { std::alloc::alloc(shape.layout) });
                            let item_poke = unsafe { Poke::unchecked_new(item_data, shape) };

                            stack.push_front(StackItem::AfterListItem { item: item_data });
                            stack.push_front(StackItem::Value { poke: item_poke });
                        } else {
                            stack.push_front(StackItem::FinishList { pl });
                        }
                    }
                    Poke::Map(map_uninit) => {
                        trace!("Deserializing \x1b[1;36mhashmap\x1b[0m");
                        let first_key = parser.expect_object_start()?;

                        let pm = map_uninit.init(None).unwrap_or_else(|_| {
                            panic!("Failed to initialize map"); // TODO: map errors
                        });

                        if let Some(key) = first_key {
                            let value_shape = pm.def.v;

                            stack.push_front(StackItem::FinishMap { pm });

                            let value_data =
                                OpaqueUninit::new(unsafe { std::alloc::alloc(value_shape.layout) });
                            let value_poke =
                                unsafe { Poke::unchecked_new(value_data, value_shape) };

                            stack.push_front(StackItem::Value { poke: value_poke });

                            stack.push_front(StackItem::AfterMapEntry { key });
                        } else {
                            stack.push_front(StackItem::FinishMap { pm });
                        }
                    }
                    Poke::Enum(pe) => {
                        trace!("Deserializing \x1b[1;36menum\x1b[0m");
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
            StackItem::StructField { key } => {
                trace!("Processing struct key: \x1b[1;33m{}\x1b[0m", key);

                let ps = match stack.front_mut().unwrap() {
                    StackItem::FinishStruct { ps } => ps,
                    _ => unreachable!(),
                };

                match ps.field_by_name(&key) {
                    Ok((index, field_poke)) => {
                        trace!("Found field, it's at index: \x1b[1;33m{index}\x1b[0m");

                        stack.push_front(StackItem::AfterStructField { index });

                        stack.push_front(StackItem::Value { poke: field_poke });
                    }
                    Err(_) => {
                        trace!("No field named \x1b[1;36m{}\x1b[0m", key);
                        return Err(parser.make_error(JsonParseErrorKind::UnknownField(key)));
                    }
                }
            }
            StackItem::AfterStructField { index } => {
                trace!("After processing struct field at index: \x1b[1;33m{index}\x1b[0m");

                let ps = match stack.front_mut().unwrap() {
                    StackItem::FinishStruct { ps } => ps,
                    _ => unreachable!(),
                };

                unsafe {
                    ps.mark_initialized(index);
                }

                let next_key = parser.parse_object_key()?;
                if let Some(next_key) = next_key {
                    stack.push_front(StackItem::StructField { key: next_key });
                }
            }
            StackItem::FinishStruct { ps } => {
                trace!("Finished deserializing \x1b[1;36mstruct\x1b[0m");

                let opaque = ps.build_in_place();
                result = Some(opaque);
            }
            StackItem::AfterListItem { item } => {
                trace!("Processing array item at index");

                let pl = match stack.front_mut().unwrap() {
                    StackItem::FinishList { pl } => pl,
                    _ => unreachable!(),
                };
                unsafe {
                    pl.push(item.assume_init());
                }

                let has_next = parser.parse_array_element()?;
                if let Some(true) = has_next {
                    let item_shape = pl.def().t;
                    let item_data =
                        OpaqueUninit::new(unsafe { std::alloc::alloc(item_shape.layout) });
                    let item_poke = unsafe { Poke::unchecked_new(item_data, item_shape) };

                    stack.push_front(StackItem::AfterListItem { item: item_data });
                    stack.push_front(StackItem::Value { poke: item_poke });
                }
            }
            StackItem::FinishList { pl } => {
                trace!("Finished deserializing \x1b[1;36marray\x1b[0m");
                let opaque = pl.build_in_place();
                result = Some(opaque);
            }
            StackItem::AfterMapEntry { key } => {
                trace!("Processing hashmap key: \x1b[1;33m{}\x1b[0m", key);

                let pm = match stack.front_mut().unwrap() {
                    StackItem::FinishMap { pm } => pm,
                    _ => unreachable!(),
                };

                let key_data = OpaqueUninit::new(unsafe { std::alloc::alloc(pm.def.k.layout) });
                let key_poke = unsafe { Poke::unchecked_new(key_data, pm.def.k) };
                let scalar_key_poke = key_poke.into_scalar();
                scalar_key_poke.parse(&key).unwrap(); // TODO: map errors

                let value_data = OpaqueUninit::new(unsafe { std::alloc::alloc(pm.def.v.layout) });
                let value_poke = unsafe { Poke::unchecked_new(value_data, pm.def.v) };

                let value_opaque = result.take().expect("Expected a value result");
                unsafe {
                    pm.insert(key_data.assume_init(), value_opaque);
                }

                stack.push_front(StackItem::Value { poke: value_poke });

                let next_key = parser.parse_object_key()?;
                if let Some(next_key) = next_key {
                    stack.push_front(StackItem::AfterMapEntry { key: next_key });
                }
            }
            StackItem::FinishMap { pm } => {
                trace!("Finished deserializing \x1b[1;36mhashmap\x1b[0m");
                let opaque = pm.build_in_place();
                result = Some(opaque);
            }
        }
    }

    result.ok_or_else(|| {
        parser.make_error(JsonParseErrorKind::Custom(
            "Unexpected end of input".to_string(),
        ))
    })
}
