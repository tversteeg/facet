#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use facet_poke::Poke;
use facet_trait::{Facet, Opaque, OpaqueConst, ShapeExt};
use log::*;

#[cfg(test)]
mod tests;

/// Deserializes a URL encoded form data string into a value of type `T` that implements `Facet`.
///
/// # Example
///
/// ```
/// use facet_derive::Facet;
/// use facet_trait::{self as facet, Facet};
/// use facet_urlencoded::from_str;
///
/// #[derive(Debug, Facet, PartialEq)]
/// struct SearchParams {
///     query: String,
///     page: u64,
/// }
///
/// let query_string = "query=rust+programming&page=2";
///
/// let params: SearchParams = from_str(query_string).expect("Failed to parse URL encoded data");
/// assert_eq!(params, SearchParams { query: "rust programming".to_string(), page: 2 });
/// ```
pub fn from_str<T: Facet>(urlencoded: &str) -> Result<T, UrlEncodedError> {
    let (poke, _guard) = Poke::alloc::<T>();
    let opaque = from_str_opaque(poke, urlencoded)?;
    Ok(unsafe { opaque.read::<T>() })
}

/// Deserializes a URL encoded form data string into an `Opaque` value.
/// 
/// This is the lower-level function that works with `Poke` directly.
fn from_str_opaque<'mem>(
    poke: Poke<'mem>,
    urlencoded: &str,
) -> Result<Opaque<'mem>, UrlEncodedError> {
    trace!("Starting URL encoded form data deserialization");

    // Parse the URL encoded string into key-value pairs
    let pairs = form_urlencoded::parse(urlencoded.as_bytes());

    // Create a map to store the parsed key-value pairs
    let mut pairs_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for (key, value) in pairs {
        pairs_map.insert(key.to_string(), value.to_string());
    }

    match poke {
        Poke::Struct(mut ps) => {
            trace!("Deserializing struct");

            for (key, value) in pairs_map {
                trace!("Processing field key: {}", key);

                match ps.field_by_name(&key) {
                    Ok((index, field_poke)) => {
                        match field_poke {
                            Poke::Scalar(ps) => {
                                if ps.shape().is_type::<String>() {
                                    let s = value;
                                    let opaque = OpaqueConst::from_ref(&s);
                                    unsafe { ps.put(opaque) };
                                    core::mem::forget(s);
                                } else if ps.shape().is_type::<u64>() {
                                    match value.parse::<u64>() {
                                        Ok(num) => {
                                            let opaque = OpaqueConst::from_ref(&num);
                                            unsafe { ps.put(opaque) };
                                        },
                                        Err(_) => {
                                            return Err(UrlEncodedError::InvalidNumber(key, value));
                                        }
                                    }
                                } else {
                                    warn!("Unsupported scalar type: {}", ps.shape());
                                    return Err(UrlEncodedError::UnsupportedType(
                                        format!("{}", ps.shape())
                                    ));
                                }
                            },
                            _ => {
                                error!("Unsupported field type");
                                return Err(UrlEncodedError::UnsupportedShape(
                                    "Unsupported field type".to_string()
                                ));
                            }
                        }
                        unsafe { ps.mark_initialized(index) };
                    },
                    Err(_) => {
                        warn!("Unknown field: {}", key);
                        // Skip unknown fields
                    }
                };
            }

            trace!("Finished deserializing struct");
            Ok(ps.build_in_place())
        },
        _ => {
            error!("Unsupported root type");
            Err(UrlEncodedError::UnsupportedShape(
                "Unsupported root type".to_string()
            ))
        }
    }
}

/// Errors that can occur during URL encoded form data deserialization.
#[derive(Debug)]
#[non_exhaustive]
pub enum UrlEncodedError {
    /// The field value couldn't be parsed as a number.
    InvalidNumber(String, String),
    /// The shape is not supported for deserialization.
    UnsupportedShape(String),
    /// The type is not supported for deserialization.
    UnsupportedType(String),
}

impl core::fmt::Display for UrlEncodedError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            UrlEncodedError::InvalidNumber(field, value) => {
                write!(f, "Invalid number for field '{}': '{}'", field, value)
            }
            UrlEncodedError::UnsupportedShape(shape) => {
                write!(f, "Unsupported shape: {}", shape)
            }
            UrlEncodedError::UnsupportedType(ty) => {
                write!(f, "Unsupported type: {}", ty)
            }
        }
    }
}

impl std::error::Error for UrlEncodedError {}
