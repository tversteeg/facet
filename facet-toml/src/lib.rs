#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod error;
mod to_scalar;

use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use error::AnyErr;
use facet_core::{Def, Facet};
use facet_reflect::{ScalarType, Wip};
use toml_edit::{DocumentMut, Item, TomlError};

/// Deserializes a TOML string into a value of type `T` that implements `Facet`.
pub fn from_str<T: Facet>(toml: &str) -> Result<T, AnyErr> {
    let wip = Wip::alloc::<T>();
    let wip = from_str_value(wip, toml)?;
    let heap_value = wip.build().map_err(|e| AnyErr(e.to_string()))?;
    heap_value
        .materialize::<T>()
        .map_err(|e| AnyErr(e.to_string()))
}

fn from_str_value<'a>(wip: Wip<'a>, toml: &str) -> Result<Wip<'a>, AnyErr> {
    let docs: DocumentMut = toml.parse().map_err(|e| TomlError::to_string(&e))?;
    deserialize_item(wip, docs.as_item())
}

fn deserialize_item<'a>(wip: Wip<'a>, item: &Item) -> Result<Wip<'a>, AnyErr> {
    let shape = wip.shape();
    match shape.def {
        Def::Scalar(_) => deserialize_as_scalar(wip, item),
        Def::List(_) => deserialize_as_list(wip, item),
        Def::Map(_) => deserialize_as_map(wip, item),
        Def::Struct(_) => deserialize_as_struct(wip, item),
        Def::Enum(_) => deserialize_as_enum(wip, item),
        Def::SmartPointer(_) => deserialize_as_smartpointer(wip, item),
        _ => Err(AnyErr(format!("Unsupported type: {:?}", shape))),
    }
}

fn deserialize_as_struct<'a>(mut wip: Wip<'a>, item: &Item) -> Result<Wip<'a>, AnyErr> {
    // Parse as the inner struct type if item is a single value and the struct is a unit struct
    if item.is_value() {
        // Only allow unit structs
        let shape = wip.shape();
        if let Def::Struct(def) = shape.def {
            if def.fields.len() > 1 {
                return Err(AnyErr(
                    "Failed trying to parse a single value as a struct with multiple fields".into(),
                ));
            }
        }

        let field_index = 0;
        wip = wip
            .field(field_index)
            .map_err(|e| AnyErr(format!("Unit struct is missing value: {}", e)))?;
        wip = deserialize_item(wip, item)?;
        wip = wip.pop().map_err(|e| AnyErr(e.to_string()))?;
        return Ok(wip);
    }

    // Otherwise we expect a table
    let table = item.as_table_like().ok_or_else(|| {
        AnyErr(format!(
            "Expected table like structure, got {}",
            item.type_name()
        ))
    })?;

    for (k, v) in table.iter() {
        let field_index = wip
            .field_index(k)
            .ok_or_else(|| AnyErr(format!("Field '{}' not found", k)))?;
        wip = wip
            .field(field_index)
            .map_err(|e| AnyErr(format!("Field '{}' error: {}", k, e)))?;
        wip = deserialize_item(wip, v)
            .map_err(|e| AnyErr(format!("Error deserializing field '{}': {}", k, e)))?;
        wip = wip.pop().map_err(|e| AnyErr(e.to_string()))?;
    }

    Ok(wip)
}

fn deserialize_as_enum<'a>(wip: Wip<'a>, item: &Item) -> Result<Wip<'a>, AnyErr> {
    if item.is_value() {
        let variant_name = item
            .as_str()
            .ok_or_else(|| AnyErr(format!("Expected string, got: {}", item.type_name())))?;

        // Use variant_named to select the variant by name
        wip.variant_named(variant_name).map_err(|e| {
            AnyErr(format!(
                "Error selecting enum variant '{}': {}",
                variant_name, e
            ))
        })
    } else {
        // TODO: Handle non-unit enum variants
        Err(AnyErr("Non-unit enum variants not yet supported".into()))
    }
}

fn deserialize_as_list<'a>(mut wip: Wip<'a>, item: &Item) -> Result<Wip<'a>, AnyErr> {
    todo!()
}

fn deserialize_as_map<'a>(mut wip: Wip<'a>, item: &Item) -> Result<Wip<'a>, AnyErr> {
    todo!()
}

fn deserialize_as_smartpointer<'a>(mut wip: Wip<'a>, item: &Item) -> Result<Wip<'a>, AnyErr> {
    todo!()
}

fn deserialize_as_scalar<'a>(mut wip: Wip<'a>, item: &Item) -> Result<Wip<'a>, AnyErr> {
    match ScalarType::try_from_shape(wip.shape())
        .ok_or_else(|| format!("Unsupported scalar type: {}", wip.shape()))?
    {
        ScalarType::Bool => {
            wip = wip
                .put(to_scalar::boolean(item)?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        #[cfg(feature = "std")]
        ScalarType::String => {
            wip = wip
                .put(to_scalar::string(item)?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        #[cfg(feature = "std")]
        ScalarType::CowStr => {
            todo!()
        }
        ScalarType::F32 => {
            wip = wip
                .put(to_scalar::number::<f32>(item)?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        ScalarType::F64 => {
            wip = wip
                .put(to_scalar::number::<f64>(item)?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        ScalarType::U8 => {
            wip = wip
                .put(to_scalar::number::<u8>(item)?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        ScalarType::U16 => {
            wip = wip
                .put(to_scalar::number::<u16>(item)?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        ScalarType::U32 => {
            wip = wip
                .put(to_scalar::number::<u32>(item)?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        ScalarType::U64 => {
            wip = wip
                .put(to_scalar::number::<u64>(item)?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        ScalarType::USize => {
            wip = wip
                .put(to_scalar::number::<usize>(item)?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        ScalarType::I8 => {
            wip = wip
                .put(to_scalar::number::<i8>(item)?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        ScalarType::I16 => {
            wip = wip
                .put(to_scalar::number::<i16>(item)?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        ScalarType::I32 => {
            wip = wip
                .put(to_scalar::number::<i32>(item)?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        ScalarType::I64 => {
            wip = wip
                .put(to_scalar::number::<i64>(item)?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        ScalarType::ISize => {
            wip = wip
                .put(to_scalar::number::<isize>(item)?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        #[cfg(feature = "std")]
        ScalarType::SocketAddr => {
            wip = wip
                .put(to_scalar::from_str::<std::net::SocketAddr>(
                    item,
                    "socket address",
                )?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        ScalarType::IpAddr => {
            wip = wip
                .put(to_scalar::from_str::<IpAddr>(item, "ip address")?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        ScalarType::Ipv4Addr => {
            wip = wip
                .put(to_scalar::from_str::<Ipv4Addr>(item, "ipv4 address")?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        ScalarType::Ipv6Addr => {
            wip = wip
                .put(to_scalar::from_str::<Ipv6Addr>(item, "ipv6 address")?)
                .map_err(|e| AnyErr(e.to_string()))?
        }
        _ => return Err(AnyErr(format!("Unsupported scalar type: {}", wip.shape()))),
    }

    Ok(wip)
}
