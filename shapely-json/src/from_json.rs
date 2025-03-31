use crate::parser::{JsonParseErrorKind, JsonParseErrorWithContext, JsonParser};
use shapely::{error, trace, warn, Partial};

pub fn from_json<'input>(
    partial: &mut Partial,
    json: &'input str,
) -> Result<(), JsonParseErrorWithContext<'input>> {
    use shapely::{Innards, Scalar};

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
            Innards::Scalar(scalar) => {
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
            Innards::Struct { .. } => {
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
            Innards::Tuple { .. } => {
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
            Innards::TupleStruct { .. } => {
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
            Innards::Array { item_shape, .. } => {
                trace!("Deserializing \x1b[1;36marray\x1b[0m");

                // Parse array start
                parser.expect_array_start()?;

                // Get the array slot to push items into
                let mut array_slot = partial.array_slot().expect("Array slot");

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
            // Add support for other shapes (Array, Transparent) as needed
            _ => {
                error!("Unsupported shape: {:?}", shape.innards);
                return Err(parser.make_error(JsonParseErrorKind::Custom(format!(
                    "Unsupported shape: {:?}",
                    shape.innards
                ))));
            }
        }
        Ok(())
    }

    deserialize_value(&mut parser, partial)
}
