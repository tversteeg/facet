use parser::{JsonParseErrorKind, JsonParseErrorWithContext, JsonParser};
use shapely::{Shape, ShapeUninit};

#[doc(hidden)]
pub mod log;

mod parser;

#[cfg(test)]
mod tests;

pub fn from_json<'input>(
    target: &mut ShapeUninit,
    json: &'input str,
) -> Result<(), JsonParseErrorWithContext<'input>> {
    use shapely::{Innards, Scalar};

    trace!("Starting JSON deserialization");
    let mut parser = JsonParser::new(json);

    fn deserialize_value<'input>(
        parser: &'input mut JsonParser,
        target: &mut ShapeUninit,
        shape: &Shape,
    ) -> Result<(), JsonParseErrorWithContext<'input>> {
        trace!("Deserializing value with shape:\n{:?}", shape);
        match &shape.innards {
            Innards::Scalar(scalar) => {
                match scalar {
                    Scalar::String => {
                        trace!("Deserializing String");
                        let s = parser.parse_string()?;
                        trace!("Deserialized String: {}", s);
                        unsafe {
                            *(target as *mut String) = s;
                        }
                    }
                    Scalar::U64 => {
                        trace!("Deserializing U64");
                        let n = parser.parse_u64()?;
                        unsafe {
                            *(target as *mut u64) = n;
                        }
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

                    if let Some(field) = fields.iter().find(|f| f.name == key) {
                        let field_schema = (field.shape)();
                        trace!("Deserializing field: {}", field.name);
                        let mut field_error = None;
                        unsafe {
                            let slot = manipulator.slot(target, field);
                            slot.set(&mut |field_ptr| {
                                if let Err(err) =
                                    deserialize_value(parser, field_ptr, &field_schema)
                                {
                                    field_error = Some(err);
                                }
                            });
                        }
                        if let Some(err) = field_error {
                            return Err(err);
                        }
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

    let result = deserialize_value(&mut parser, target, &schema);
    if result.is_ok() {
        trace!("JSON deserialization completed successfully");
    } else {
        error!("JSON deserialization failed: {:?}", result);
    }
    result
}
