// use proc_macro::TokenStream;
use unsynn::*;

keyword! {
    Pub = "pub";
    Struct = "struct";
}

unsynn! {
    enum Enum {
        Two(Plus, Plus, Dot),
        One(Plus, Dot),
        TwoS { a: Minus, b: Minus, c: Dot},
        OneS { a: Minus, b: Dot },
    }

    struct AttributeInner {
        name: Ident,
        attr: ParenthesisGroupContaining<Vec<Ident>>,
    }

    struct Attribute {
        _pound: Pound,
        body: BracketGroupContaining<AttributeInner>,
    }

    struct StructLike {
        attributes: Vec<Attribute>,
        _pub: Option<Pub>,
        _kw_struct: Struct,
        name: Ident,
        body: BraceGroupContaining<CommaDelimitedVec<FieldLike>>,
    }

    struct FieldLike {
        _pub: Option<Pub>,
        name: Ident,
        _colon: Colon,
        typ: Ident,
    }
}

#[proc_macro]
pub fn parse_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let mut i = input.to_token_iter();
    let parsed: Enum = i.parse().unwrap();
    let dbg_s = format!("{parsed:#?}");
    let s = format!("println!(\"Parsed: {{}}\", {dbg_s:?});");
    s.into_token_stream().into()
}

#[proc_macro]
pub fn parse_struct_like(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let mut i = input.to_token_iter();
    let parsed: StructLike = i.parse().unwrap();
    let dbg_s = format!("{parsed:#?}");
    let s = format!("println!(\"Parsed: {{}}\", {dbg_s:?});");
    s.into_token_stream().into()
}

#[proc_macro_derive(Shapely)]
pub fn shapely_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let mut i = input.to_token_iter();
    let parsed: StructLike = i.parse().unwrap();
    let dbg_s = format!("{parsed:#?}");

    let struct_def = format!(
        "impl {name} {{
            fn get_parsed_structure() -> &'static str {{
                {dbg_s:?}
            }}
        }}",
        name = parsed.name
    );
    struct_def.into_token_stream().into()
}
