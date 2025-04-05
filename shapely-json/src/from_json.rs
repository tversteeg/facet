use crate::parser::{JsonParseErrorKind, JsonParseErrorWithContext, JsonParser};

use log::trace;
use shapely_poke::Poke;
use shapely_trait::{Opaque, OpaqueConst, OpaqueUninit, ShapeExt as _};

/// Deserialize a `Poke` object from a JSON string.
pub fn from_json<'input, 'mem>(
    poke: Poke<'mem>,
    json: &'input str,
) -> Result<Opaque<'mem>, JsonParseErrorWithContext<'input>> {
    trace!("Starting JSON deserialization");
    let mut parser = JsonParser::new(json);
    deserialize_value(&mut parser, poke)
}

fn deserialize_value<'input, 'mem>(
    parser: &mut JsonParser<'input>,
    poke: Poke<'mem>,
) -> Result<Opaque<'mem>, JsonParseErrorWithContext<'input>> {
    let shape = poke.shape();
    trace!("Deserializing {:?}", shape);

    let opaque = match poke {
        Poke::Scalar(pv) => {
            trace!("Deserializing \x1b[1;36mscalar\x1b[0m");
            if pv.shape().is_type::<String>() {
                let s = parser.parse_string()?;
                let data = unsafe { pv.put(OpaqueConst::from_ref(&s)) };
                std::mem::forget(s);
                data
            } else if pv.shape().is_type::<u64>() {
                let n = parser.parse_u64()?;
                unsafe { pv.put(OpaqueConst::from_ref(&n)) }
            } else {
                panic!("Unknown scalar shape: {}", pv.shape());
            }
        }
        Poke::Struct(mut ps) => {
            trace!("Deserializing \x1b[1;36mstruct\x1b[0m");

            let mut first = true;
            while let Some(key) = if first {
                first = false;
                parser.expect_object_start()?
            } else {
                parser.parse_object_key()?
            } {
                trace!("Processing struct key: \x1b[1;33m{}\x1b[0m", key);
                let index = match ps.field_by_name(&key) {
                    Ok((index, field_poke)) => {
                        deserialize_value(parser, field_poke)?;
                        index
                    }
                    Err(_) => {
                        return Err(parser.make_error(JsonParseErrorKind::UnknownField(key)));
                    }
                };
                unsafe { ps.mark_initialized(index) };
            }
            trace!("Finished deserializing \x1b[1;36mstruct\x1b[0m");
            ps.build_in_place()
        }
        Poke::List(list_uninit) => {
            trace!("Deserializing \x1b[1;36marray\x1b[0m");

            // Parse array start
            parser.expect_array_start()?;

            // Initialize the list with no size hint
            let mut pl = list_uninit.init(None).unwrap_or_else(|_| {
                panic!("Failed to initialize list");
            });

            let mut index = 0;
            while let Some(has_element) = parser.parse_array_element()? {
                if !has_element {
                    break;
                }

                trace!("Processing array item at index: \x1b[1;33m{}\x1b[0m", index);

                let data = OpaqueUninit::new(unsafe { std::alloc::alloc(shape.layout) });
                let item_poke = unsafe { Poke::unchecked_new(data, shape) };

                // Deserialize the item
                deserialize_value(parser, item_poke)?;

                // Add the item to the list
                unsafe {
                    pl.push(data.assume_init());
                }

                index += 1;
            }

            trace!(
                "Finished deserializing \x1b[1;36marray\x1b[0m with {} items",
                index
            );
            pl.build_in_place()
        }
        Poke::Map(pm) => {
            trace!("Deserializing \x1b[1;36mhashmap\x1b[0m");

            // Parse object start and get first key if it exists
            let first_key = parser.expect_object_start()?;

            // Initialize the map with no size hint
            let mut pm = pm
                .init(None)
                .unwrap_or_else(|_| panic!("Failed to initialize map")); // TODO: map errors

            // Process each key-value pair in the JSON object
            let mut current_key = first_key;
            while let Some(key) = current_key {
                trace!("Processing hashmap key: \x1b[1;33m{}\x1b[0m", key);

                // Create a poke for the key (string type)
                let key_data = OpaqueUninit::new(unsafe { std::alloc::alloc(pm.def.k.layout) });
                let key_poke = unsafe { Poke::unchecked_new(key_data, pm.def.k) };
                let scalar_key_poke = key_poke.into_scalar();
                scalar_key_poke.parse(&key).unwrap(); // TODO: map errors

                // Create a poke for the value based on map def
                let value_data = OpaqueUninit::new(unsafe { std::alloc::alloc(pm.def.v.layout) });
                let value_poke = unsafe { Poke::unchecked_new(value_data, pm.def.v) };

                // Deserialize the value
                deserialize_value(parser, value_poke)?;

                // Insert the key-value pair into the hashmap
                unsafe {
                    pm.insert(key_data.assume_init(), value_data.assume_init());
                }

                // Get the next key
                current_key = parser.parse_object_key()?;
            }

            trace!("Finished deserializing \x1b[1;36mhashmap\x1b[0m");
            pm.build_in_place()
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
            pe.build_in_place()
        }
    };
    Ok(opaque)
}
