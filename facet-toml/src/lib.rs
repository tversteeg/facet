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
use log::trace;
use toml_edit::{DocumentMut, Item, TomlError};

/// Deserializes a TOML string into a value of type `T` that implements `Facet`.
pub fn from_str<T: Facet>(toml: &str) -> Result<T, AnyErr> {
    let (poke, _guard) = PokeUninit::alloc::<T>();
    let opaque = from_str_opaque(poke, toml)?;

    Ok(unsafe { opaque.read::<T>() })
}

fn from_str_opaque<'mem>(poke: PokeUninit<'mem>, toml: &str) -> Result<Opaque<'mem>, AnyErr> {
    trace!("Starting TOML deserialization");
    let docs: DocumentMut = toml.parse().map_err(|e| TomlError::to_string(&e))?;
    let opaque = deserialize_item(poke, docs.as_item())?;
    trace!("Finished TOML deserialization");

    Ok(opaque)
}

fn deserialize_item<'mem>(poke: PokeUninit<'mem>, item: &Item) -> Result<Opaque<'mem>, AnyErr> {
    trace!(
        "Deserializing TOML \x1b[1;3m{}\x1b[0m into \x1b[1;2m{}\x1b[0m",
        item.type_name(),
        poke.shape()
    );

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
    trace!("Deserializing \x1b[1;36mstruct\x1b[0m");

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

    trace!("Finished deserializing \x1b[1;36mstruct\x1b[0m");

    Ok(poke.build_in_place())
}

fn deserialize_as_enum<'mem>(
    poke: PokeEnumNoVariant<'mem>,
    item: &Item,
) -> Result<Opaque<'mem>, AnyErr> {
    trace!("Deserializing \x1b[1;36menum\x1b[0m");

    match item {
        Item::None => todo!(),
        Item::Value(value) => {
            // A value can be an inline table, so parse it as such
            if let Some(inline_table) = value.as_inline_table() {
                if let Some((key, field)) = inline_table.iter().next() {
                    trace!("Entering inline table with key \x1b[1;3m{key}\x1b[0m");

                    if inline_table.len() > 1 {
                        return Err(
                            "Cannot parse enum from inline table because it got multiple fields"
                                .into(),
                        );
                    } else {
                        return build_enum_from_variant_name(
                            poke,
                            key,
                            // TODO: remove clone
                            &Item::Value(field.clone()),
                        );
                    }
                } else {
                    return Err(
                        "Inline table doesn't have any fields to parse into enum variant".into(),
                    );
                }
            }

            let variant_name = value
                .as_str()
                .ok_or_else(|| format!("Expected string, got: {}", value.type_name()))?;

            build_enum_from_variant_name(poke, variant_name, item)
        }
        Item::Table(table) => {
            if let Some((key, field)) = table.iter().next() {
                trace!("Entering table with key \x1b[1;3m{key}\x1b[0m");

                if table.len() > 1 {
                    Err("Cannot parse enum from inline table because it got multiple fields".into())
                } else {
                    build_enum_from_variant_name(poke, key, field)
                }
            } else {
                Err("Inline table doesn't have any fields to parse into enum variant".into())
            }
        }
        Item::ArrayOfTables(_array_of_tables) => todo!(),
    }
}

fn build_enum_from_variant_name<'mem>(
    poke: PokeEnumNoVariant<'mem>,
    variant_name: &str,
    item: &Item,
) -> Result<Opaque<'mem>, AnyErr> {
    let variant = poke
        .variant_by_name(variant_name)
        .ok_or_else(|| format!("Enum does not have a variant named '{variant_name}'"))?;

    // Copy the kind so we can look at it later
    let kind = variant.kind;

    let mut init = poke
        .set_variant_by_name(variant_name)
        .map_err(|err| err.to_string())?;

    match kind {
        // No need to do anything, we can just set the variant since it's a unit enum
        VariantKind::Unit => {
            trace!("Enum variant \x1b[1;2m{variant_name}\x1b[0m is a unit");
        }
        // Initialize the enum tuple fields
        VariantKind::Tuple { fields } => {
            trace!("Enum variant \x1b[1;2m{variant_name}\x1b[0m is a tuple");

            for field in fields {
                let (index, init_field) = init
                    .field_by_name(field.name)
                    .map_err(|e| format!("Field by name on enum does not exist: {e}"))?;

                let sub_item = if item.is_table_like() {
                    let table = item.as_table_like().unwrap();

                    // Get the indexed item from the table
                    table
                        .get(&index.to_string())
                        .ok_or_else(|| format!("Field '{index}' does not exist"))?
                } else if item.is_value() {
                    // Try to get the TOML value as a table to extract the field
                    let Some(item) = item.as_value() else {
                        return Err(format!("TOML {} is not a table", item.type_name()).into());
                    };

                    // TODO: remove clone
                    &Item::Value(item.clone())
                } else {
                    todo!()
                };

                // Deserialize the field
                deserialize_item(init_field, sub_item)?;

                unsafe { init.mark_initialized(index) };
            }
        }
        // Initialize the enum struct fields
        VariantKind::Struct { fields } => {
            trace!("Enum variant \x1b[1;2m{variant_name}\x1b[0m is a struct");

            for field in fields {
                let (index, init_field) = init
                    .field_by_name(field.name)
                    .map_err(|e| format!("Field by name on enum does not exist: {e}"))?;

                // Try to get the TOML value as a table to extract the field
                let Some(item) = item.as_table_like() else {
                    return Err(format!("TOML {} is not a table", item.type_name()).into());
                };

                // Try to get the TOML field matching the Rust name
                let Some(field) = item.get(field.name) else {
                    return Err(format!("TOML field '{}' not found", field.name).into());
                };

                // Deserialize the field
                deserialize_item(init_field, field)?;

                unsafe { init.mark_initialized(index) };
            }
        }
        _ => panic!("unimplemented variant kind"),
    }

    trace!("Finished deserializing \x1b[1;36menum\x1b[0m");

    Ok(init.build_in_place())
}

fn deserialize_as_option<'mem>(
    _poke: PokeOptionUninit<'mem>,
    _item: &Item,
) -> Result<Opaque<'mem>, AnyErr> {
    trace!("Deserializing \x1b[1;36moption\x1b[0m");

    todo!()
}

fn deserialize_as_list<'mem>(
    _poke: PokeListUninit<'mem>,
    _item: &Item,
) -> Result<Opaque<'mem>, AnyErr> {
    trace!("Deserializing \x1b[1;36marray\x1b[0m");

    todo!()
}

fn deserialize_as_map<'mem>(
    _poke: PokeMapUninit<'mem>,
    _item: &Item,
) -> Result<Opaque<'mem>, AnyErr> {
    trace!("Deserializing \x1b[1;36mmap\x1b[0m");

    todo!()
}

fn deserialize_as_scalar<'mem>(
    poke: PokeValueUninit<'mem>,
    item: &Item,
) -> Result<Opaque<'mem>, AnyErr> {
    trace!("Deserializing \x1b[1;36mscalar\x1b[0m");

    let opaque = match poke
        .scalar_type()
        .ok_or_else(|| format!("Unsupported scalar type: {}", poke.shape()))?
    {
        ScalarType::Bool => poke.put(to_scalar::boolean(item)?),

        #[cfg(feature = "std")]
        ScalarType::String => poke.put(to_scalar::string(item)?),

        #[cfg(feature = "std")]
        ScalarType::CowStr => poke.put(std::borrow::Cow::Owned(to_scalar::string(item)?)),

        ScalarType::F32 => poke.put(to_scalar::number::<f32>(item)?),
        ScalarType::F64 => poke.put(to_scalar::number::<f64>(item)?),
        ScalarType::U8 => poke.put(to_scalar::number::<u8>(item)?),
        ScalarType::U16 => poke.put(to_scalar::number::<u16>(item)?),
        ScalarType::U32 => poke.put(to_scalar::number::<u32>(item)?),
        ScalarType::U64 => poke.put(to_scalar::number::<u64>(item)?),
        ScalarType::USize => poke.put(to_scalar::number::<usize>(item)?),

        ScalarType::I8 => poke.put(to_scalar::number::<i8>(item)?),
        ScalarType::I16 => poke.put(to_scalar::number::<i16>(item)?),
        ScalarType::I32 => poke.put(to_scalar::number::<i32>(item)?),
        ScalarType::I64 => poke.put(to_scalar::number::<i64>(item)?),
        ScalarType::ISize => poke.put(to_scalar::number::<isize>(item)?),

        #[cfg(feature = "std")]
        ScalarType::SocketAddr => poke.put(to_scalar::from_str::<std::net::SocketAddr>(
            item,
            "socket address",
        )?),
        ScalarType::IpAddr => poke.put(to_scalar::from_str::<IpAddr>(item, "ip address")?),
        ScalarType::Ipv4Addr => poke.put(to_scalar::from_str::<Ipv4Addr>(item, "ipv4 address")?),
        ScalarType::Ipv6Addr => poke.put(to_scalar::from_str::<Ipv6Addr>(item, "ipv6 address")?),

        _ => return Err(format!("Unsupported scalar type: {}", poke.shape()).into()),
    };

    trace!("Finished deserializing \x1b[1;36mscalar\x1b[0m");

    Ok(opaque)
}
