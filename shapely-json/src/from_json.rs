use crate::parser::{JsonParseErrorKind, JsonParseErrorWithContext, JsonParser};
use shapely::{Def, OpaqueConst, OpaqueUninit, Poke, ShapeDebug, trace};

/// Deserialize a `Poke` object from a JSON string.
pub fn from_json<'input>(
    poke: Poke<'_>,
    json: &'input str,
) -> Result<(), JsonParseErrorWithContext<'input>> {
    trace!("Starting JSON deserialization");
    let mut parser = JsonParser::new(json);

    fn deserialize_value<'input>(
        parser: &mut JsonParser<'input>,
        poke: Poke<'_>,
    ) -> Result<(), JsonParseErrorWithContext<'input>> {
        let shape = poke.shape();
        trace!("Deserializing value with shape:\n{:?}", ShapeDebug(shape));

        match &shape.def {
            Def::Scalar { .. } => {
                let pv = poke.into_scalar();
                trace!("Deserializing \x1b[1;36mscalar\x1b[0m");

                trace!(
                    "pv.shape.is_type::<String>() = {}",
                    pv.shape.is_type::<String>()
                );
                trace!("pv.shape.is_type::<u64>() = {}", pv.shape.is_type::<u64>());

                if pv.shape.is_type::<String>() {
                    trace!("Deserializing string (pv shape = {})", pv.shape);
                    let s = parser.parse_string()?;
                    unsafe { pv.put(OpaqueConst::from_ref(&s)) };
                    std::mem::forget(s);
                } else if pv.shape.is_type::<u64>() {
                    trace!("Deserializing u64 (pv shape = {})", pv.shape);
                    let n = parser.parse_u64()?;
                    unsafe { pv.put(OpaqueConst::from_ref(&n)) };
                } else {
                    panic!("Unknown scalar shape: {}", pv.shape);
                }
            }
            Def::Struct(_struct_def) | Def::TupleStruct(_struct_def) => {
                trace!("Deserializing \x1b[1;36mstruct\x1b[0m");
                let mut struct_poke = poke.into_struct();

                let mut first = true;
                while let Some(key) = if first {
                    first = false;
                    parser.expect_object_start()?
                } else {
                    parser.parse_object_key()?
                } {
                    trace!("Processing struct key: \x1b[1;33m{}\x1b[0m", key);
                    match struct_poke.field_by_name(&key) {
                        Ok(field_poke) => {
                            deserialize_value(parser, field_poke)?;
                        }
                        Err(_) => {
                            return Err(parser.make_error(JsonParseErrorKind::UnknownField(key)));
                        }
                    }
                }
                trace!("Finished deserializing \x1b[1;36mstruct\x1b[0m");
            }
            Def::Tuple(struct_def) => {
                trace!("Deserializing \x1b[1;36mtuple\x1b[0m");
                let mut struct_poke = poke.into_struct();

                // Parse array start
                parser.expect_array_start()?;

                let mut index = 0;
                while let Some(has_element) = parser.parse_array_element()? {
                    if !has_element {
                        break;
                    }

                    trace!("Processing tuple index: \x1b[1;33m{}\x1b[0m", index);
                    if index < struct_def.fields.len() {
                        let field_poke = struct_poke.field(index).unwrap(); // TODO: map errors
                        deserialize_value(parser, field_poke)?;
                    } else {
                        return Err(parser.make_error(JsonParseErrorKind::Custom(format!(
                            "Tuple index out of bounds: {}",
                            index
                        ))));
                    }

                    index += 1;
                }

                trace!("Finished deserializing \x1b[1;36mtuple\x1b[0m");
            }
            Def::List(_list_def) => {
                trace!("Deserializing \x1b[1;36marray\x1b[0m");

                // Parse array start
                parser.expect_array_start()?;

                // Initialize the list with no size hint
                let mut list_poke = poke.into_list().init(None).unwrap_or_else(|_| {
                    panic!("Failed to initialize list");
                });

                let mut index = 0;
                while let Some(has_element) = parser.parse_array_element()? {
                    if !has_element {
                        break;
                    }

                    trace!("Processing array item at index: \x1b[1;33m{}\x1b[0m", index);

                    let data = OpaqueUninit::new(unsafe { std::alloc::alloc(shape.layout) });
                    let item_poke = unsafe { Poke::from_opaque_uninit(data, shape) };

                    // Deserialize the item
                    deserialize_value(parser, item_poke)?;

                    // Add the item to the list
                    unsafe {
                        list_poke.push(data.assume_init());
                    }

                    index += 1;
                }

                trace!(
                    "Finished deserializing \x1b[1;36marray\x1b[0m with {} items",
                    index
                );
            }
            Def::Map(map_def) => {
                trace!("Deserializing \x1b[1;36mhashmap\x1b[0m");

                // Parse object start and get first key if it exists
                let first_key = parser.expect_object_start()?;

                // Initialize the map with no size hint
                let mut map_poke = poke
                    .into_map()
                    .init(None)
                    .unwrap_or_else(|_| panic!("Failed to initialize map")); // TODO: map errors

                // Process each key-value pair in the JSON object
                let mut current_key = first_key;
                while let Some(key) = current_key {
                    trace!("Processing hashmap key: \x1b[1;33m{}\x1b[0m", key);

                    // Create a poke for the key (string type)
                    let key_data =
                        OpaqueUninit::new(unsafe { std::alloc::alloc(map_def.k.layout) });
                    let key_poke = unsafe { Poke::from_opaque_uninit(key_data, map_def.k) };
                    let scalar_key_poke = key_poke.into_scalar();
                    scalar_key_poke.parse(&key).unwrap(); // TODO: map errors

                    // Create a poke for the value based on map def
                    let value_data =
                        OpaqueUninit::new(unsafe { std::alloc::alloc(map_def.v.layout) });
                    let value_poke = unsafe { Poke::from_opaque_uninit(value_data, map_def.v) };

                    // Deserialize the value
                    deserialize_value(parser, value_poke)?;

                    // Insert the key-value pair into the hashmap
                    unsafe {
                        map_poke.insert(key_data.assume_init(), value_data.assume_init());
                    }

                    // Get the next key
                    current_key = parser.parse_object_key()?;
                }

                trace!("Finished deserializing \x1b[1;36mhashmap\x1b[0m");
            }
            Def::Enum(_enum_def) => {
                trace!("Deserializing \x1b[1;36menum\x1b[0m");
                // Assuming enums are serialized as JSON strings representing the variant name
                let variant_str = parser.parse_string()?;
                let enum_poke = poke.into_enum();

                enum_poke.set_variant_by_name(&variant_str).map_err(|_| {
                    parser.make_error(JsonParseErrorKind::Custom(format!(
                        "Invalid enum variant: {}",
                        variant_str
                    )))
                })?;

                trace!("Finished deserializing \x1b[1;36menum\x1b[0m");
            }
        }
        Ok(())
    }

    deserialize_value(&mut parser, poke)
}
