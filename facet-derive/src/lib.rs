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
    enum TypeDecl {
        Struct(Struct),
        Enum(Enum),
        TupleStruct(TupleStruct)
    }

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
        inner: ParenthesisGroupContaining<FacetInner>,
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
        body: Option<BraceGroupContaining<CommaDelimitedVec<StructField>>>,
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
        Tuple(TupleVariant),
        Unit(UnitVariant),
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

    // Parse as TypeDecl
    match i.parse::<TypeDecl>() {
        Ok(TypeDecl::Struct(parsed)) => process_struct::process_struct(parsed),
        Ok(TypeDecl::TupleStruct(parsed)) => process_tuple_struct::process_tuple_struct(parsed),
        Ok(TypeDecl::Enum(parsed)) => process_enum::process_enum(parsed),
        Err(err) => {
            panic!(
                "Could not parse input as struct, tuple struct, or enum: {}\nError: {:?}",
                input, err
            );
        }
    }
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
