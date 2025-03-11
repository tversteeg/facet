use unsynn::*;

keyword! {
    Pub = "pub";
    Struct = "struct";
}

unsynn! {
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

#[proc_macro_derive(Shapely)]
pub fn shapely_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let mut i = input.to_token_iter();
    let parsed: StructLike = i.parse().unwrap();

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
