use crate::parser::{JsonParseErrorKind, JsonParseErrorWithContext, JsonParser};
use shapely::{Partial, Shapely as _, error, trace, warn};

/// Deserialize a `Partial` object from a JSON string.
pub fn from_json<'input>(
    partial: &mut Partial,
    json: &'input str,
) -> Result<(), JsonParseErrorWithContext<'input>> {
    use shapely::{Def, Scalar};

    trace!("Starting JSON deserialization");
    let mut parser = JsonParser::new(json);

    fn deserialize_value<'input>(
        parser: &mut JsonParser<'input>,
        partial: &mut Partial,
    ) -> Result<(), JsonParseErrorWithContext<'input>> {
        let shape_desc = partial.shape();
        let shape = shape_desc.get();
        trace!("Deserializing value with shape:\n{:?}", shape);

        match &shape.innards {
            Def::Scalar(scalar) => {
                let slot = partial.scalar_slot().expect("Scalar slot");
                trace!("Deserializing \x1b[1;36mscalar\x1b[0m, \x1b[1;35m{scalar:?}\x1b[0m");

                match scalar {
                    Scalar::String => slot.fill(parser.parse_string()?),
                    Scalar::U8 => slot.fill(parser.parse_u8()?),
                    Scalar::U16 => slot.fill(parser.parse_u16()?),
                    Scalar::U32 => slot.fill(parser.parse_u32()?),
                    Scalar::U64 => slot.fill(parser.parse_u64()?),
                    Scalar::I8 => slot.fill(parser.parse_i8()?),
                    Scalar::I16 => slot.fill(parser.parse_i16()?),
                    Scalar::I32 => slot.fill(parser.parse_i32()?),
                    Scalar::I64 => slot.fill(parser.parse_i64()?),
                    Scalar::F32 => slot.fill(parser.parse_f32()?),
                    Scalar::F64 => slot.fill(parser.parse_f64()?),
                    Scalar::Boolean => slot.fill(parser.parse_bool()?),
                    _ => {
                        warn!("Unsupported scalar type: {:?}", scalar);
                        return Err(parser.make_error(JsonParseErrorKind::Custom(format!(
                            "Unsupported scalar type: {:?}",
                            scalar
                        ))));
                    }
                }
            }
            Def::Struct { .. } => {
                trace!("Deserializing \x1b[1;36mstruct\x1b[0m");

                let mut first = true;
                while let Some(key) = if first {
                    first = false;
                    parser.expect_object_start()?
                } else {
                    parser.parse_object_key()?
                } {
                    trace!("Processing struct key: \x1b[1;33m{}\x1b[0m", key);
                    let slot = partial
                        .slot_by_name(&key)
                        .map_err(|_| parser.make_error(JsonParseErrorKind::UnknownField(key)))?;
                    let mut partial_field = Partial::alloc(slot.shape());
                    deserialize_value(parser, &mut partial_field)?;
                    slot.fill_from_partial(partial_field);
                }
                trace!("Finished deserializing \x1b[1;36mstruct\x1b[0m");

                // TODO: this would be a good place to decide what to do about unset fields? Is this
                // where we finally get to use `set_default`?
            }
            Def::Tuple { .. } => {
                trace!("Deserializing \x1b[1;36mtuple\x1b[0m");

                // Parse array start
                parser.expect_array_start()?;

                let mut index = 0;
                while let Some(has_element) = parser.parse_array_element()? {
                    if !has_element {
                        break;
                    }

                    let field_name = index.to_string();
                    trace!("Processing tuple index: \x1b[1;33m{}\x1b[0m", field_name);

                    let slot = partial.slot_by_name(&field_name).map_err(|_| {
                        parser.make_error(JsonParseErrorKind::Custom(format!(
                            "Tuple index out of bounds: {}",
                            index
                        )))
                    })?;

                    let mut partial_field = Partial::alloc(slot.shape());
                    deserialize_value(parser, &mut partial_field)?;
                    slot.fill_from_partial(partial_field);

                    index += 1;
                }

                trace!("Finished deserializing \x1b[1;36mtuple\x1b[0m");
            }
            Def::TupleStruct { .. } => {
                trace!("Deserializing \x1b[1;36mtuple struct\x1b[0m");

                // Parse array start
                parser.expect_array_start()?;

                let mut index = 0;
                while let Some(has_element) = parser.parse_array_element()? {
                    if !has_element {
                        break;
                    }

                    let field_name = index.to_string();
                    trace!(
                        "Processing tuple struct index: \x1b[1;33m{}\x1b[0m",
                        field_name
                    );

                    let slot = partial.slot_by_name(&field_name).map_err(|_| {
                        parser.make_error(JsonParseErrorKind::Custom(format!(
                            "Tuple struct index out of bounds: {}",
                            index
                        )))
                    })?;

                    let mut partial_field = Partial::alloc(slot.shape());
                    deserialize_value(parser, &mut partial_field)?;
                    slot.fill_from_partial(partial_field);

                    index += 1;
                }

                trace!("Finished deserializing \x1b[1;36mtuple struct\x1b[0m");
            }
            Def::List { item_shape, .. } => {
                trace!("Deserializing \x1b[1;36marray\x1b[0m");

                // Parse array start
                parser.expect_array_start()?;

                // Get the array slot to push items into (no size hint in JSON unfortunately)
                let mut array_slot = partial.list_writer(None).expect("Array slot");

                let mut index = 0;
                while let Some(has_element) = parser.parse_array_element()? {
                    if !has_element {
                        break;
                    }

                    trace!("Processing array item at index: \x1b[1;33m{}\x1b[0m", index);

                    // Create a partial for the item
                    let mut item_partial = Partial::alloc(*item_shape);

                    // Deserialize the item
                    deserialize_value(parser, &mut item_partial)?;

                    // Add the item to the array
                    array_slot.push(item_partial);

                    index += 1;
                }

                trace!(
                    "Finished deserializing \x1b[1;36marray\x1b[0m with {} items",
                    index
                );
            }
            Def::Map { value_shape, .. } => {
                trace!("Deserializing \x1b[1;36mhashmap\x1b[0m");

                // Parse object start and get first key if it exists
                let first_key = parser.expect_object_start()?;

                // Get the hashmap slot to insert key-value pairs into
                let mut hashmap_slot = partial.map_writer(None).expect("HashMap slot");

                // Process each key-value pair in the JSON object
                let mut current_key = first_key;
                while let Some(key) = current_key {
                    trace!("Processing hashmap key: \x1b[1;33m{}\x1b[0m", key);

                    // Create a partial for the key (string type)
                    let mut key_partial = Partial::alloc(String::shape_desc());
                    key_partial.scalar_slot().expect("String slot").fill(key);

                    // Create a partial for the value
                    let mut value_partial = Partial::alloc(*value_shape);

                    // Deserialize the value
                    deserialize_value(parser, &mut value_partial)?;

                    // Insert the key-value pair into the hashmap
                    hashmap_slot.insert(key_partial, value_partial);

                    // Get the next key
                    current_key = parser.parse_object_key()?;
                }

                trace!("Finished deserializing \x1b[1;36mhashmap\x1b[0m");
            }
            // Add support for other shapes (Array, Transparent) as needed
            _ => {
                error!(
                    "Don't know how to parse this shape as JSON: {:?}",
                    shape.innards
                );
                return Err(parser.make_error(JsonParseErrorKind::Custom(format!(
                    "Don't know how to parse this shape as JSON: {:?}",
                    shape.innards
                ))));
            }
        }
        Ok(())
    }

    deserialize_value(&mut parser, partial)
}
