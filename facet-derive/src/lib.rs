#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod process_enum;
mod process_struct;
mod process_tuple_struct;

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
    KFacet = "facet";
    KSensitive = "sensitive";
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
        Facet(FacetAttr),
        Doc(DocInner),
        Repr(ReprInner),
        Any(Vec<TokenTree>)
    }

    struct FacetAttr {
        _facet: KFacet,
        _sensitive: ParenthesisGroupContaining<FacetInner>,
    }

    enum FacetInner {
        Sensitive(KSensitive),
        Other(Vec<TokenTree>)
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

/// Derive the Facet trait for structs, tuple structs, and enums.
///
/// This uses unsynn, so it's light, but it _will_ choke on some Rust syntax because...
/// there's a lot of Rust syntax.
#[proc_macro_derive(Facet, attributes(facet))]
pub fn facet_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let mut i = input.to_token_iter();

    // Try to parse as struct first
    if let Ok(parsed) = i.parse::<Struct>() {
        return process_struct::process_struct(parsed);
    }
    let struct_tokens_left = i.count();

    // Try to parse as tuple struct
    i = input.to_token_iter(); // Reset iterator
    if let Ok(parsed) = i.parse::<TupleStruct>() {
        return process_tuple_struct::process_tuple_struct(parsed);
    }
    let tuple_struct_tokens_left = i.count();

    // Try to parse as enum
    i = input.to_token_iter(); // Reset iterator
    if let Ok(parsed) = i.parse::<Enum>() {
        return process_enum::process_enum(parsed);
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

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

impl core::fmt::Display for ConstOrMut {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ConstOrMut::Const(_) => write!(f, "const"),
            ConstOrMut::Mut(_) => write!(f, "mut"),
        }
    }
}

impl core::fmt::Display for Lifetime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "'{}", self.name)
    }
}

impl core::fmt::Display for Expr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Expr::Integer(int) => write!(f, "{}", int.value()),
        }
    }
}
