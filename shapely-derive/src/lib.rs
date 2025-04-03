#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use unsynn::*;

keyword! {
    KPub = "pub";
    KStruct = "struct";
    KEnum = "enum";
    KDoc = "doc";
    KRepr = "repr";
    KCrate = "crate";
    KConst = "const";
    KMut = "mut";
}

operator! {
    Eq = "=";
    Semi = ";";
    Apostrophe = "'";
    DoubleSemicolon = "::";
}

unsynn! {
    enum Vis {
        Pub(KPub),
        PubCrate(Cons<KPub, ParenthesisGroupContaining<KCrate>>),
    }

    struct Attribute {
        _pound: Pound,
        body: BracketGroupContaining<AttributeInner>,
    }

    enum AttributeInner {
        Doc(DocInner),
        Repr(ReprInner),
        Any(Vec<TokenTree>)
    }

    struct DocInner {
        _kw_doc: KDoc,
        _eq: Eq,
        value: LiteralString,
    }

    struct ReprInner {
        _kw_repr: KRepr,
        attr: ParenthesisGroupContaining<Ident>,
    }

    struct Struct {
        // Skip any doc attributes by consuming them
        attributes: Vec<Attribute>,
        _vis: Option<Vis>,
        _kw_struct: KStruct,
        name: Ident,
        body: BraceGroupContaining<CommaDelimitedVec<StructField>>,
    }

    struct Lifetime {
        _apostrophe: Apostrophe,
        name: Ident,
    }

    enum Expr {
        Integer(LiteralInteger),
    }

    enum Type {
        Path(PathType),
        Tuple(ParenthesisGroupContaining<CommaDelimitedVec<Box<Type>>>),
        Slice(BracketGroupContaining<Box<Type>>),
        Bare(BareType),
    }

    struct PathType {
        prefix: Ident,
        _doublesemi: DoubleSemicolon,
        rest: Box<Type>,
    }

    struct BareType {
        name: Ident,
        generic_params: Option<GenericParams>,
    }

    struct GenericParams {
        _lt: Lt,
        params: CommaDelimitedVec<Type>,
        _gt: Gt,
    }

    enum ConstOrMut {
        Const(KConst),
        Mut(KMut),
    }

    struct StructField {
        attributes: Vec<Attribute>,
        _vis: Option<Vis>,
        name: Ident,
        _colon: Colon,
        typ: Type,
    }

    struct TupleStruct {
        // Skip any doc attributes by consuming them
        attributes: Vec<Attribute>,
        _vis: Option<Vis>,
        _kw_struct: KStruct,
        name: Ident,
        body: ParenthesisGroupContaining<CommaDelimitedVec<TupleField>>,
    }

    struct TupleField {
        attributes: Vec<Attribute>,
        vis: Option<Vis>,
        typ: Type,
    }

    struct Enum {
        // Skip any doc attributes by consuming them
        attributes: Vec<Attribute>,
        _pub: Option<KPub>,
        _kw_enum: KEnum,
        name: Ident,
        body: BraceGroupContaining<CommaDelimitedVec<EnumVariantLike>>,
    }

    enum EnumVariantLike {
        Unit(UnitVariant),
        Tuple(TupleVariant),
        Struct(StructVariant),
    }

    struct UnitVariant {
        attributes: Vec<Attribute>,
        name: Ident,
    }

    struct TupleVariant {
        // Skip any doc comments on variants
        attributes: Vec<Attribute>,
        name: Ident,
        _paren: ParenthesisGroupContaining<CommaDelimitedVec<TupleField>>,
    }

    struct StructVariant {
        // Skip any doc comments on variants
        _doc_attributes: Vec<Attribute>,
        name: Ident,
        _brace: BraceGroupContaining<CommaDelimitedVec<StructField>>,
    }
}

/// Derive the [`shapely_core::Shapely`] trait for structs, tuple structs, and enums.
///
/// This uses unsynn, so it's light, but it _will_ choke on some Rust syntax because...
/// there's a lot of Rust syntax.
#[proc_macro_derive(Shapely)]
pub fn shapely_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let mut i = input.to_token_iter();

    // Try to parse as struct first
    if let Ok(parsed) = i.parse::<Struct>() {
        return process_struct(parsed);
    }
    let struct_tokens_left = i.count();

    // Try to parse as tuple struct
    i = input.to_token_iter(); // Reset iterator
    if let Ok(parsed) = i.parse::<TupleStruct>() {
        return process_tuple_struct(parsed);
    }
    let tuple_struct_tokens_left = i.count();

    // Try to parse as enum
    i = input.to_token_iter(); // Reset iterator
    if let Ok(parsed) = i.parse::<Enum>() {
        return process_enum(parsed);
    }
    let enum_tokens_left = i.count();

    let mut msg = format!(
        "Could not parse input as struct, tuple struct, or enum: {}",
        input
    );

    // Find which parsing left the fewest tokens
    let min_tokens_left = struct_tokens_left
        .min(tuple_struct_tokens_left)
        .min(enum_tokens_left);

    // Parse again for the one with fewest tokens left and show remaining tokens
    if min_tokens_left == struct_tokens_left {
        i = input.to_token_iter();
        let err = i.parse::<Struct>().err();
        msg = format!(
            "{}\n====> Error parsing struct: {:?}\n====> Remaining tokens after struct parsing: {}",
            msg,
            err,
            i.collect::<TokenStream>()
        );
    } else if min_tokens_left == tuple_struct_tokens_left {
        i = input.to_token_iter();
        let err = i.parse::<TupleStruct>().err();
        msg = format!(
            "{}\n====> Error parsing tuple struct: {:?}\n====> Remaining tokens after tuple struct parsing: {}",
            msg,
            err,
            i.collect::<TokenStream>()
        );
    } else {
        i = input.to_token_iter();
        let err = i.parse::<Enum>().err();
        msg = format!(
            "{}\n====> Error parsing enum: {:?}\n====> Remaining tokens after enum parsing: {}",
            msg,
            err,
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
fn process_struct(parsed: Struct) -> proc_macro::TokenStream {
    let struct_name = parsed.name.to_string();
    let fields = parsed
        .body
        .content
        .0
        .iter()
        .map(|field| field.value.name.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    // Generate the impl
    let output = format!(
        r#"
#[automatically_derived]
impl shapely::Shapely for {struct_name} {{
    const SHAPE: &'static shapely::Shape = &const {{
        shapely::Shape {{
            layout: std::alloc::Layout::new::<Self>(),
            vtable: &shapely::ValueVTable {{
                type_name: |f, _opts| std::fmt::Write::write_str(f, "{struct_name}"),
                display: if shapely_core::impls!(Self: std::fmt::Display) {{
                    Some(|data, f| {{
                        use shapely::spez::*;
                        (&&Spez(unsafe {{ data.as_ref::<Self>() }})).spez_display(f)
                    }})
                }} else {{
                    None
                }},
                debug: if shapely_core::impls!(Self: std::fmt::Debug) {{
                    Some(|data, f| {{
                        use shapely::spez::*;
                        (&&Spez(unsafe {{ data.as_ref::<Self>() }})).spez_debug(f)
                    }})
                }} else {{
                    None
                }},
                default_in_place: if shapely_core::impls!(Self: std::default::Default) {{
                    Some(|target| {{
                        use shapely::spez::*;
                        let dummy_val: std::mem::MaybeUninit<Self> = std::mem::MaybeUninit::zeroed();
                        Some((&&Spez(unsafe {{ dummy_val.assume_init() }})).spez_default_in_place(target))
                    }})
                }} else {{
                    None
                }},
                clone_into: if shapely_core::impls!(Self: std::clone::Clone) {{
                    Some(|src, dst| {{
                        use shapely::spez::*;
                        Some((&&Spez(unsafe {{ src.as_ref::<Self>() }})).spez_clone_into(dst))
                    }})
                }} else {{
                    None
                }},
                eq: if shapely_core::impls!(Self: std::cmp::PartialEq) {{
                    Some(|left, right| {{
                        use shapely::spez::*;
                        (&&Spez(unsafe {{ left.as_ref::<Self>() }}))
                            .spez_eq(&&Spez(unsafe {{ right.as_ref::<Self>() }}))
                    }})
                }} else {{
                    None
                }},
                ord: if shapely_core::impls!(Self: std::cmp::Ord) {{
                    Some(|left, right| {{
                        use shapely::spez::*;
                        (&&Spez(unsafe {{ left.as_ref::<Self>() }}))
                            .spez_cmp(&&Spez(unsafe {{ right.as_ref::<Self>() }}))
                    }})
                }} else {{
                    None
                }},
                hash: if shapely_core::impls!(Self: std::hash::Hash) {{
                    Some(|value, hasher_this, hasher_write_fn| {{
                        use shapely::spez::*;
                        use shapely::vtable::HasherProxy;
                        (&&Spez(unsafe {{ value.as_ref::<Self>() }}))
                            .spez_hash(&mut unsafe {{ HasherProxy::new(hasher_this, hasher_write_fn) }})
                    }})
                }} else {{
                    None
                }},
                drop_in_place: Some(|data| unsafe {{ data.drop_in_place::<Self>() }}),
                parse: None,
                try_from: None,
            }},
            def: shapely::Def::Struct(shapely::StructDef {{
                fields: shapely::struct_fields!({struct_name}, ({fields})),
            }}),
        }}
    }};
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
#[automatically_derived]
impl shapely::Shapely for {struct_name} {{
    const SHAPE: &'static shapely::Shape = &const {{
        shapely::Shape {{
            layout: std::alloc::Layout::new::<Self>(),
            vtable: &shapely::ValueVTable {{
                type_name: |f, _opts| std::fmt::Write::write_str(f, "{struct_name}"),
                display: if shapely_core::impls!(Self: std::fmt::Display) {{
                    Some(|data, f| {{
                        use shapely::spez::*;
                        (&&Spez(unsafe {{ data.as_ref::<Self>() }})).spez_display(f)
                    }})
                }} else {{
                    None
                }},
                debug: if shapely_core::impls!(Self: std::fmt::Debug) {{
                    Some(|data, f| {{
                        use shapely::spez::*;
                        (&&Spez(unsafe {{ data.as_ref::<Self>() }})).spez_debug(f)
                    }})
                }} else {{
                    None
                }},
                default_in_place: if shapely_core::impls!(Self: std::default::Default) {{
                    Some(|target| {{
                        use shapely::spez::*;
                        let dummy_val: std::mem::MaybeUninit<Self> = std::mem::MaybeUninit::zeroed();
                        Some((&&Spez(unsafe {{ dummy_val.assume_init() }})).spez_default_in_place(target))
                    }})
                }} else {{
                    None
                }},
                clone_into: if shapely_core::impls!(Self: std::clone::Clone) {{
                    Some(|src, dst| {{
                        use shapely::spez::*;
                        Some((&&Spez(unsafe {{ src.as_ref::<Self>() }})).spez_clone_into(dst))
                    }})
                }} else {{
                    None
                }},
                eq: if shapely_core::impls!(Self: std::cmp::PartialEq) {{
                    Some(|left, right| {{
                        use shapely::spez::*;
                        (&&Spez(unsafe {{ left.as_ref::<Self>() }}))
                            .spez_eq(&&Spez(unsafe {{ right.as_ref::<Self>() }}))
                    }})
                }} else {{
                    None
                }},
                ord: if shapely_core::impls!(Self: std::cmp::Ord) {{
                    Some(|left, right| {{
                        use shapely::spez::*;
                        (&&Spez(unsafe {{ left.as_ref::<Self>() }}))
                            .spez_cmp(&&Spez(unsafe {{ right.as_ref::<Self>() }}))
                    }})
                }} else {{
                    None
                }},
                hash: if shapely_core::impls!(Self: std::hash::Hash) {{
                    Some(|value, hasher_this, hasher_write_fn| {{
                        use shapely::spez::*;
                        use shapely::vtable::HasherProxy;
                        (&&Spez(unsafe {{ value.as_ref::<Self>() }}))
                            .spez_hash(&mut unsafe {{ HasherProxy::new(hasher_this, hasher_write_fn) }})
                    }})
                }} else {{
                    None
                }},
                drop_in_place: Some(|data| unsafe {{ data.drop_in_place::<Self>() }}),
                parse: None,
                try_from: None,
            }},
            def: shapely::Def::TupleStruct(shapely::StructDef {{
                fields: shapely::struct_fields!({struct_name}, ({fields_str})),
            }}),
        }}
    }};
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
fn process_enum(parsed: Enum) -> proc_macro::TokenStream {
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
                    .map(|field| field.value.typ.to_string())
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
                        let typ = field.value.typ.to_string();
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
#[automatically_derived]
impl shapely::Shapely for {enum_name} {{
    const SHAPE: &'static shapely::Shape = &const {{
        shapely::Shape {{
            layout: std::alloc::Layout::new::<Self>(),
            vtable: &shapely::ValueVTable {{
                type_name: |f, _opts| std::fmt::Write::write_str(f, "{enum_name}"),
                display: if shapely_core::impls!(Self: std::fmt::Display) {{
                    Some(|data, f| {{
                        use shapely::spez::*;
                        (&&Spez(unsafe {{ data.as_ref::<Self>() }})).spez_display(f)
                    }})
                }} else {{
                    None
                }},
                debug: if shapely_core::impls!(Self: std::fmt::Debug) {{
                    Some(|data, f| {{
                        use shapely::spez::*;
                        (&&Spez(unsafe {{ data.as_ref::<Self>() }})).spez_debug(f)
                    }})
                }} else {{
                    None
                }},
                default_in_place: if shapely_core::impls!(Self: std::default::Default) {{
                    Some(|target| {{
                        use shapely::spez::*;
                        let dummy_val: std::mem::MaybeUninit<Self> = std::mem::MaybeUninit::zeroed();
                        Some((&&Spez(unsafe {{ dummy_val.assume_init() }})).spez_default_in_place(target))
                    }})
                }} else {{
                    None
                }},
                clone_into: if shapely_core::impls!(Self: std::clone::Clone) {{
                    Some(|src, dst| {{
                        use shapely::spez::*;
                        Some((&&Spez(unsafe {{ src.as_ref::<Self>() }})).spez_clone_into(dst))
                    }})
                }} else {{
                    None
                }},
                eq: if shapely_core::impls!(Self: std::cmp::PartialEq) {{
                    Some(|left, right| {{
                        use shapely::spez::*;
                        (&&Spez(unsafe {{ left.as_ref::<Self>() }}))
                            .spez_eq(&&Spez(unsafe {{ right.as_ref::<Self>() }}))
                    }})
                }} else {{
                    None
                }},
                ord: if shapely_core::impls!(Self: std::cmp::Ord) {{
                    Some(|left, right| {{
                        use shapely::spez::*;
                        (&&Spez(unsafe {{ left.as_ref::<Self>() }}))
                            .spez_cmp(&&Spez(unsafe {{ right.as_ref::<Self>() }}))
                    }})
                }} else {{
                    None
                }},
                hash: if shapely_core::impls!(Self: std::hash::Hash) {{
                    Some(|value, hasher_this, hasher_write_fn| {{
                        use shapely::spez::*;
                        use shapely::vtable::HasherProxy;
                        (&&Spez(unsafe {{ value.as_ref::<Self>() }}))
                            .spez_hash(&mut unsafe {{ HasherProxy::new(hasher_this, hasher_write_fn) }})
                    }})
                }} else {{
                    None
                }},
                drop_in_place: Some(|data| unsafe {{ data.drop_in_place::<Self>() }}),
                parse: None,
                try_from: None,
            }},
            def: shapely::Def::Enum(shapely::EnumDef {{
                variants: shapely::enum_variants!({enum_name}, [{variants}]),
                repr: shapely::EnumRepr::{repr_type},
            }}),
        }}
    }};
}}
        "#
    );
    output.into_token_stream().into()
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Path(path) => {
                write!(f, "{}::{}", path.prefix, path.rest)
            }
            Type::Tuple(tuple) => {
                write!(f, "(")?;
                for (i, typ) in tuple.content.0.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", typ.value)?;
                }
                write!(f, ")")
            }
            Type::Slice(slice) => {
                write!(f, "[{}]", slice.content)
            }
            Type::Bare(ident) => {
                write!(f, "{}", ident.name)?;
                if let Some(generic_params) = &ident.generic_params {
                    write!(f, "<")?;
                    for (i, param) in generic_params.params.0.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", param.value)?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
        }
    }
}

impl std::fmt::Display for ConstOrMut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstOrMut::Const(_) => write!(f, "const"),
            ConstOrMut::Mut(_) => write!(f, "mut"),
        }
    }
}

impl std::fmt::Display for Lifetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}", self.name)
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Integer(int) => write!(f, "{}", int.value()),
        }
    }
}
