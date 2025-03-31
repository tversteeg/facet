use unsynn::*;

keyword! {
    Pub = "pub";
    Struct = "struct";
    Enum = "enum";
    Doc = "doc";
    Repr = "repr";
}

operator! {
    Eq = "=";
}

unsynn! {
    struct DocAttributeInner {
        doc: Doc,
        _eq: Eq,
        value: LiteralString,
    }

    struct DocAttribute {
        _pound: Pound,
        body: BracketGroupContaining<DocAttributeInner>,
    }

    struct AttributeInnerRepr {
        _kw_repr: Repr,
        attr: ParenthesisGroupContaining<Ident>,
    }

    enum AttributeInner {
        Repr(AttributeInnerRepr),
        Any(Vec<TokenTree>)
    }

    struct Attribute {
        _pound: Pound,
        body: BracketGroupContaining<AttributeInner>,
    }

    struct StructLike {
        // Skip any doc attributes by consuming them
        _doc_attributes: Vec<Skip<DocAttribute>>,
        attributes: Vec<Attribute>,
        _pub: Option<Pub>,
        _kw_struct: Struct,
        name: Ident,
        body: BraceGroupContaining<CommaDelimitedVec<FieldLike>>,
    }

    struct TupleStruct {
        // Skip any doc attributes by consuming them
        _doc_attributes: Vec<Skip<DocAttribute>>,
        attributes: Vec<Attribute>,
        _pub: Option<Pub>,
        _kw_struct: Struct,
        name: Ident,
        body: ParenthesisGroupContaining<CommaDelimitedVec<FieldLike>>,
    }

    struct FieldLike {
        // Skip any doc comments on fields
        _doc_attributes: Vec<Skip<DocAttribute>>,
        _pub: Option<Pub>,
        name: Ident,
        _colon: Option<Colon>,
        typ: Option<Ident>,
    }

    struct EnumLike {
        // Skip any doc attributes by consuming them
        _doc_attributes: Vec<Skip<DocAttribute>>,
        attributes: Vec<Attribute>,
        _pub: Option<Pub>,
        _kw_enum: Enum,
        name: Ident,
        body: BraceGroupContaining<CommaDelimitedVec<EnumVariantLike>>,
    }

    enum EnumVariantLike {
        Unit(UnitVariant),
        Tuple(TupleVariant),
        Struct(StructVariant),
    }

    struct UnitVariant {
        // Skip any doc comments on variants
        _doc_attributes: Vec<Skip<DocAttribute>>,
        name: Ident,
    }

    struct TupleVariant {
        // Skip any doc comments on variants
        _doc_attributes: Vec<Skip<DocAttribute>>,
        name: Ident,
        _paren: ParenthesisGroupContaining<CommaDelimitedVec<FieldLike>>,
    }

    struct StructVariant {
        // Skip any doc comments on variants
        _doc_attributes: Vec<Skip<DocAttribute>>,
        name: Ident,
        _brace: BraceGroupContaining<CommaDelimitedVec<FieldLike>>,
    }
}

#[proc_macro_derive(Shapely)]
pub fn shapely_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let mut i = input.to_token_iter();

    // Try to parse as struct first
    if let Ok(parsed) = i.parse::<StructLike>() {
        return process_struct(parsed);
    }
    let struct_like_tokens_left = i.count();

    // Try to parse as tuple struct
    i = input.to_token_iter(); // Reset iterator
    if let Ok(parsed) = i.parse::<TupleStruct>() {
        return process_tuple_struct(parsed);
    }
    let tuple_struct_like_tokens_left = i.count();

    // Try to parse as enum
    i = input.to_token_iter(); // Reset iterator
    if let Ok(parsed) = i.parse::<EnumLike>() {
        return process_enum(parsed);
    }
    let enum_like_tokens_left = i.count();

    let mut msg = format!(
        "Could not parse input as struct, tuple struct, or enum: {}",
        input
    );

    // Find which parsing left the fewest tokens
    let min_tokens_left = struct_like_tokens_left
        .min(tuple_struct_like_tokens_left)
        .min(enum_like_tokens_left);

    // Parse again for the one with fewest tokens left and show remaining tokens
    if min_tokens_left == struct_like_tokens_left {
        i = input.to_token_iter();
        i.parse::<StructLike>().err();
        msg = format!(
            "{}\nRemaining tokens after struct parsing: {}",
            msg,
            i.collect::<TokenStream>()
        );
    } else if min_tokens_left == tuple_struct_like_tokens_left {
        i = input.to_token_iter();
        i.parse::<TupleStruct>().err();
        msg = format!(
            "{}\nRemaining tokens after tuple struct parsing: {}",
            msg,
            i.collect::<TokenStream>()
        );
    } else {
        i = input.to_token_iter();
        i.parse::<EnumLike>().err();
        msg = format!(
            "{}\nRemaining tokens after enum parsing: {}",
            msg,
            i.collect::<TokenStream>()
        );
    }

    // If we get here, couldn't parse as struct, tuple struct, or enum
    panic!("{msg}");
}

/// Processes a regular struct to implement Shapely
///
/// Example input:
/// ```rust
/// struct Blah {
///     foo: u32,
///     bar: String,
/// }
/// ```
fn process_struct(parsed: StructLike) -> proc_macro::TokenStream {
    let struct_name = parsed.name.to_string();
    let fields = parsed
        .body
        .content
        .0
        .iter()
        .map(|field| field.value.name.to_string())
        .collect::<Vec<String>>();

    // Create the fields string for struct_fields! macro
    let fields_str = fields.join(", ");

    // Generate the impl
    let output = format!(
        r#"
            impl shapely::Shapely for {struct_name} {{
                fn shape() -> shapely::Shape {{
                    shapely::Shape {{
                        name: |f| std::fmt::Write::write_str(f, "{struct_name}"),
                        typeid: shapely::mini_typeid::of::<Self>(),
                        layout: std::alloc::Layout::new::<Self>(),
                        innards: shapely::Innards::Struct {{
                            fields: shapely::struct_fields!({struct_name}, ({fields_str})),
                        }},
                        set_to_default: None,
                        drop_in_place: Some(|ptr| unsafe {{ std::ptr::drop_in_place(ptr as *mut Self) }}),
                    }}
                }}
            }}
        "#
    );
    output.into_token_stream().into()
}

/// Processes a tuple struct to implement Shapely
///
/// Example input:
/// ```rust
/// struct Point(f32, f32);
/// ```
fn process_tuple_struct(parsed: TupleStruct) -> proc_macro::TokenStream {
    let struct_name = parsed.name.to_string();

    // Generate field names for tuple elements (0, 1, 2, etc.)
    let fields = parsed
        .body
        .content
        .0
        .iter()
        .enumerate()
        .map(|(idx, _)| idx.to_string())
        .collect::<Vec<String>>();

    // Create the fields string for struct_fields! macro
    let fields_str = fields.join(", ");

    // Generate the impl
    let output = format!(
        r#"
            impl shapely::Shapely for {struct_name} {{
                fn shape() -> shapely::Shape {{
                    shapely::Shape {{
                        name: |f| std::fmt::Write::write_str(f, "{struct_name}"),
                        typeid: shapely::mini_typeid::of::<Self>(),
                        layout: std::alloc::Layout::new::<Self>(),
                        innards: shapely::Innards::Struct {{
                            fields: shapely::struct_fields!({struct_name}, ({fields_str})),
                        }},
                        set_to_default: None,
                        drop_in_place: Some(|ptr| unsafe {{ std::ptr::drop_in_place(ptr as *mut Self) }}),
                    }}
                }}
            }}
        "#
    );
    output.into_token_stream().into()
}

/// Processes an enum to implement Shapely
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
fn process_enum(parsed: EnumLike) -> proc_macro::TokenStream {
    let enum_name = parsed.name.to_string();

    // Check for explicit repr attribute
    let has_repr = parsed
        .attributes
        .iter()
        .any(|attr| matches!(attr.body.content, AttributeInner::Repr(_)));

    if !has_repr {
        return r#"compile_error!("Enums must have an explicit representation (e.g. #[repr(u8)]) to be used with Shapely")"#
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
                format!("shapely::enum_unit_variant!({enum_name}, {variant_name})")
            }
            EnumVariantLike::Tuple(tuple) => {
                let variant_name = tuple.name.to_string();
                let field_types = tuple
                    ._paren
                    .content
                    .0
                    .iter()
                    .map(|field| {
                        field
                            .value
                            .typ
                            .as_ref()
                            .map_or("".to_string(), |t| t.to_string())
                    })
                    .collect::<Vec<String>>()
                    .join(", ");

                format!(
                    "shapely::enum_tuple_variant!({enum_name}, {variant_name}, [{field_types}])"
                )
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
                        let typ = field
                            .value
                            .typ
                            .as_ref()
                            .map_or("".to_string(), |t| t.to_string());
                        format!("{name}: {typ}")
                    })
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("shapely::enum_struct_variant!({enum_name}, {variant_name}, {{{fields}}})")
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
            impl shapely::Shapely for {enum_name} {{
                fn shape() -> shapely::Shape {{
                    shapely::Shape {{
                        name: |f| std::fmt::Write::write_str(f, "{enum_name}"),
                        typeid: shapely::mini_typeid::of::<Self>(),
                        layout: std::alloc::Layout::new::<Self>(),
                        innards: shapely::Innards::Enum {{
                            variants: shapely::enum_variants!({enum_name}, [{variants}]),
                            repr: shapely::EnumRepr::{repr_type},
                        }},
                        set_to_default: None,
                        drop_in_place: Some(|ptr| unsafe {{ std::ptr::drop_in_place(ptr as *mut Self) }}),
                    }}
                }}
            }}
        "#
    );
    output.into_token_stream().into()
}
