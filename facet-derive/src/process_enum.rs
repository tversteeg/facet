use super::*;
use unsynn::*;

/// Processes an enum to implement Facet
///
/// Example input:
/// ```rust
/// #[repr(u8)]
/// enum Color {
///     Red,
///     Green,
///     Blue(u8, u8),
///     Custom { r: u8, g: u8, b: u8 }
/// }
/// ```
pub(crate) fn process_enum(parsed: Enum) -> proc_macro::TokenStream {
    let enum_name = parsed.name.to_string();

    // Check for explicit repr attribute
    let has_repr = parsed
        .attributes
        .iter()
        .any(|attr| matches!(attr.body.content, AttributeInner::Repr(_)));

    if !has_repr {
        return r#"compile_error!("Enums must have an explicit representation (e.g. #[repr(u8)]) to be used with Facet")"#
            .into_token_stream()
            .into();
    }

    // Process each variant
    let variants = parsed
        .body
        .content
        .0
        .iter()
        .map(|var_like| match &var_like.value {
            EnumVariantLike::Unit(unit) => {
                let variant_name = unit.name.to_string();
                format!("facet::enum_unit_variant!({enum_name}, {variant_name})")
            }
            EnumVariantLike::Tuple(tuple) => {
                let variant_name = tuple.name.to_string();
                let field_types = tuple
                    ._paren
                    .content
                    .0
                    .iter()
                    .map(|field| field.value.typ.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("facet::enum_tuple_variant!({enum_name}, {variant_name}, [{field_types}])")
            }
            EnumVariantLike::Struct(struct_var) => {
                let variant_name = struct_var.name.to_string();
                let fields = struct_var
                    ._brace
                    .content
                    .0
                    .iter()
                    .map(|field| {
                        let name = field.value.name.to_string();
                        let typ = field.value.typ.to_string();
                        format!("{name}: {typ}")
                    })
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("facet::enum_struct_variant!({enum_name}, {variant_name}, {{{fields}}})")
            }
        })
        .collect::<Vec<String>>()
        .join(", ");

    // Extract the repr type
    let mut repr_type = "Default"; // Default fallback
    for attr in &parsed.attributes {
        if let AttributeInner::Repr(repr_attr) = &attr.body.content {
            repr_type = match repr_attr.attr.content.to_string().as_str() {
                "u8" => "U8",
                "u16" => "U16",
                "u32" => "U32",
                "u64" => "U64",
                "usize" => "USize",
                "i8" => "I8",
                "i16" => "I16",
                "i32" => "I32",
                "i64" => "I64",
                "isize" => "ISize",
                _ => "Default", // Unknown repr type
            };
            break;
        }
    }

    // Generate the impl
    let output = format!(
        r#"
#[automatically_derived]
unsafe impl facet::Facet for {enum_name} {{
    const SHAPE: &'static facet::Shape = &const {{
        facet::Shape.builder()
            .id(facet::ConstTypeId::of::<{enum_name}>())
            .layout(core::alloc::Layout::new::<Self>())
            .vtable(facet::value_vtable!(
                {enum_name},
                |f, _opts| core::fmt::Write::write_str(f, "{enum_name}")
            ))
            .def(facet::Def::Enum(facet::EnumDef {{
                variants: facet::enum_variants!({enum_name}, [{variants}]),
                repr: facet::EnumRepr::{repr_type},
            }}))
            .build()
        }}
    }};
}}
        "#
    );
    output.into_token_stream().into()
}
