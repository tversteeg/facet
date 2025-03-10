use parser::{JsonParseErrorKind, JsonParseErrorWithContext, JsonParser};
use shapely::{error, trace, warn, Partial};

mod parser;

#[cfg(test)]
mod tests;

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
        let shape_desc = partial.shape_desc();
        let shape = shape_desc.get();
        trace!("Deserializing value with shape:\n{:?}", shape);

        match &shape.innards {
            Innards::Scalar(scalar) => {
                match scalar {
                    Scalar::String => {
                        trace!("Deserializing String");
                        let s = parser.parse_string()?;
                        trace!("Deserialized String: {}", s);
                        partial.scalar_slot().expect("String scalar slot").fill(s);
                    }
                    Scalar::U64 => {
                        trace!("Deserializing U64");
                        let n = parser.parse_u64()?;
                        partial.scalar_slot().expect("U64 scalar slot").fill(n);
                        trace!("Deserialized U64: {}", n);
                    }
                    // Add other scalar types as needed
                    _ => {
                        warn!("Unsupported scalar type: {:?}", scalar);
                        return Err(parser.make_error(JsonParseErrorKind::Custom(format!(
                            "Unsupported scalar type: {:?}",
                            scalar
                        ))));
                    }
                }
            }
            Innards::Struct { fields } => {
                trace!("Deserializing \x1b[1;36mstruct\x1b[0m");
                if let Some(first_key) = parser.expect_object_start()? {
                    trace!("Processing struct key: \x1b[1;33m{}\x1b[0m", first_key);
                    if let Some(field) = fields.iter().find(|f| f.name == first_key).copied() {
                        let mut partial_field = Partial::alloc(field.shape);
                        trace!("Deserializing field: \x1b[1;32m{}\x1b[0m", field.name);
                        deserialize_value(parser, &mut partial_field)?;
                        let slot = partial.slot(field).expect("Field slot");
                        slot.fill_from_partial(partial_field);
                    } else {
                        warn!("Unknown field: \x1b[1;31m{}\x1b[0m, skipping", first_key);
                        parser.skip_value()?;
                    }
                }
                while let Some(key) = parser.parse_object_key()? {
                    trace!("Processing struct key: \x1b[1;33m{}\x1b[0m", key);
                    if let Some(field) = fields.iter().find(|f| f.name == key).copied() {
                        // FIXME: we could definitely optimize this — the struct is already
                        // allocated at this stage, so we could grab the address of its field.
                        let mut partial_field = Partial::alloc(field.shape);
                        trace!("Deserializing field: \x1b[1;32m{}\x1b[0m", field.name);
                        deserialize_value(parser, &mut partial_field)?;
                        let slot = partial.slot(field).expect("Field slot");
                        slot.fill_from_partial(partial_field);
                    } else {
                        warn!("Unknown field: \x1b[1;31m{}\x1b[0m, skipping", key);
                        parser.skip_value()?;
                    }
                }
                trace!("Finished deserializing \x1b[1;36mstruct\x1b[0m");
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

        trace!(
            "Successfully deserialized value for shape: \x1b[1;32m{}\x1b[0m at address \x1b[1;34m{:?}\x1b[0m\n",
            shape.name,
            partial.addr_for_display()
        );
        Ok(())
    }

    let result = deserialize_value(&mut parser, partial);
    match &result {
        Ok(_) => {
            trace!("JSON deserialization completed successfully");
        }
        Err(e) => {
            error!("JSON deserialization failed: {}", e);
        }
    }
    result
}
