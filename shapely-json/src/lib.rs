use parser::{JsonParseErrorKind, JsonParseErrorWithContext, JsonParser};
use shapely::{Partial, Shape, ShapeDesc};

#[doc(hidden)]
pub mod log;

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
                trace!("Deserializing struct");
                parser.expect_object_start()?;
                while let Some(key) = parser.parse_object_key()? {
                    trace!("Processing struct key: {}", key);

                    if let Some(field) = fields.iter().find(|f| f.name == key).copied() {
                        // FIXME: we could _probably_ optimize this — the struct is already
                        // allocated at this stage, so we could grab the address of its field.
                        let mut partial_field = Partial::alloc(field.shape);
                        trace!("Deserializing field: {}", field.name);
                        deserialize_value(parser, &mut partial_field)?;
                        let slot = partial.slot(field).expect("Field slot");
                    } else {
                        warn!("Unknown field: {}, skipping", key);
                        parser.skip_value()?;
                    }
                }
                parser.expect_object_end()?;
                trace!("Finished deserializing Map");
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

    let result = deserialize_value(&mut parser, partial, partial.shape);
    if result.is_ok() {
        trace!("JSON deserialization completed successfully");
    } else {
        error!("JSON deserialization failed: {:?}", result);
    }
    result
}
