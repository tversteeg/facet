#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod error;
mod to_scalar;

use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use error::AnyErr;
use facet_core::{Facet, Opaque, VariantKind};
use facet_reflect::{
    PokeEnumNoVariant, PokeListUninit, PokeMapUninit, PokeOptionUninit, PokeStruct, PokeUninit,
    PokeValueUninit, ScalarType,
};
use toml_edit::{DocumentMut, Item, TomlError};

/// Deserializes a TOML string into a value of type `T` that implements `Facet`.
pub fn from_str<T: Facet>(toml: &str) -> Result<T, AnyErr> {
    let (poke, _guard) = PokeUninit::alloc::<T>();
    let opaque = from_str_opaque(poke, toml)?;
    Ok(unsafe { opaque.read::<T>() })
}

fn from_str_opaque<'mem>(poke: PokeUninit<'mem>, toml: &str) -> Result<Opaque<'mem>, AnyErr> {
    let docs: DocumentMut = toml.parse().map_err(|e| TomlError::to_string(&e))?;
    deserialize_item(poke, docs.as_item())
}

fn deserialize_item<'mem>(poke: PokeUninit<'mem>, item: &Item) -> Result<Opaque<'mem>, AnyErr> {
    match poke {
        PokeUninit::Struct(poke) => deserialize_as_struct(poke, item),
        PokeUninit::Enum(poke) => deserialize_as_enum(poke, item),
        PokeUninit::Option(poke) => deserialize_as_option(poke, item),
        PokeUninit::List(poke) => deserialize_as_list(poke, item),
        PokeUninit::Map(poke) => deserialize_as_map(poke, item),
        PokeUninit::Scalar(poke) => deserialize_as_scalar(poke, item),
        _ => todo!("unsupported poke type"),
    }
}

fn deserialize_as_struct<'mem>(
    mut poke: PokeStruct<'mem>,
    item: &Item,
) -> Result<Opaque<'mem>, AnyErr> {
    // Parse as a the inner struct type if item is a single value and the struct is a unit struct
    if item.is_value() {
        // Only allow unit structs
        if poke.def().fields.len() > 1 {
            return Err(
                "Failed trying to parse a single value as a struct with multiple fields".into(),
            );
        }

        return deserialize_item(
            poke.field(0)
                .map_err(|e| format!("Unit struct is missing value: {e}"))?,
            item,
        );
    }

    // Otherwise we expect a table
    let table = item
        .as_table_like()
        .ok_or_else(|| format!("Expected table like structure, got {}", item.type_name()))?;

    for (k, v) in table.iter() {
        let (index, field_poke) = poke
            .field_by_name(k)
            .map_err(|e| format!("Field '{}' error: {}", k, e))?;
        let _v = deserialize_item(field_poke, v)
            .map_err(|e| format!("Error deserializing field '{}': {}", k, e))?;
        unsafe {
            poke.mark_initialized(index);
        }
    }

    Ok(poke.build_in_place())
}

fn deserialize_as_enum<'mem>(
    poke: PokeEnumNoVariant<'mem>,
    item: &Item,
) -> Result<Opaque<'mem>, AnyErr> {
    // Item is a single value, try to get the variant with the name
    if item.is_value() {
        let value = item
            .as_str()
            .ok_or_else(|| format!("Expokected string, got: {}", item.type_name()))?;

        let variant = poke
            .variant_by_name(value)
            .ok_or_else(|| format!("Enum does not have a variant named '{value}'"))?;

        if variant.kind != VariantKind::Unit {
            return Err(format!("variant '{value}' is not a unit variant").into());
        }

        Ok(poke
            .set_variant_by_name(value)
            .map_err(|err| err.to_string())?
            .build_in_place())
    } else {
        todo!()
    }
}

fn deserialize_as_option<'mem>(
    _poke: PokeOptionUninit<'mem>,
    _item: &Item,
) -> Result<Opaque<'mem>, AnyErr> {
    todo!()
}

fn deserialize_as_list<'mem>(
    _poke: PokeListUninit<'mem>,
    _item: &Item,
) -> Result<Opaque<'mem>, AnyErr> {
    todo!()
}

fn deserialize_as_map<'mem>(
    _poke: PokeMapUninit<'mem>,
    _item: &Item,
) -> Result<Opaque<'mem>, AnyErr> {
    todo!()
}

fn deserialize_as_scalar<'mem>(
    poke: PokeValueUninit<'mem>,
    item: &Item,
) -> Result<Opaque<'mem>, AnyErr> {
    match poke
        .scalar_type()
        .ok_or_else(|| format!("Unsupported scalar type: {}", poke.shape()))?
    {
        ScalarType::Bool => Ok(poke.put(to_scalar::boolean(item)?)),
        #[cfg(feature = "std")]
        ScalarType::String => Ok(poke.put(to_scalar::string(item)?)),
        #[cfg(feature = "std")]
        ScalarType::CowStr => Ok(poke.put(std::borrow::Cow::Owned(to_scalar::string(item)?))),
        ScalarType::F32 => Ok(poke.put(to_scalar::number::<f32>(item)?)),
        ScalarType::F64 => Ok(poke.put(to_scalar::number::<f64>(item)?)),
        ScalarType::U8 => Ok(poke.put(to_scalar::number::<u8>(item)?)),
        ScalarType::U16 => Ok(poke.put(to_scalar::number::<u16>(item)?)),
        ScalarType::U32 => Ok(poke.put(to_scalar::number::<u32>(item)?)),
        ScalarType::U64 => Ok(poke.put(to_scalar::number::<u64>(item)?)),
        ScalarType::USize => Ok(poke.put(to_scalar::number::<usize>(item)?)),
        ScalarType::I8 => Ok(poke.put(to_scalar::number::<i8>(item)?)),
        ScalarType::I16 => Ok(poke.put(to_scalar::number::<i16>(item)?)),
        ScalarType::I32 => Ok(poke.put(to_scalar::number::<i32>(item)?)),
        ScalarType::I64 => Ok(poke.put(to_scalar::number::<i64>(item)?)),
        ScalarType::ISize => Ok(poke.put(to_scalar::number::<isize>(item)?)),
        #[cfg(feature = "std")]
        ScalarType::SocketAddr => Ok(poke.put(to_scalar::from_str::<std::net::SocketAddr>(
            item,
            "socket address",
        )?)),
        ScalarType::IpAddr => Ok(poke.put(to_scalar::from_str::<IpAddr>(item, "ip address")?)),
        ScalarType::Ipv4Addr => {
            Ok(poke.put(to_scalar::from_str::<Ipv4Addr>(item, "ipv4 address")?))
        }
        ScalarType::Ipv6Addr => {
            Ok(poke.put(to_scalar::from_str::<Ipv6Addr>(item, "ipv6 address")?))
        }
        _ => Err(format!("Unsupported scalar type: {}", poke.shape()).into()),
    }
}
