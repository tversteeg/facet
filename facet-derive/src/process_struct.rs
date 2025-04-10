use super::*;

/// Processes a regular struct to implement Facet
///
/// Example input:
/// ```rust
/// struct Blah {
///     foo: u32,
///     bar: String,
/// }
/// ```
pub(crate) fn process_struct(parsed: Struct) -> proc_macro::TokenStream {
    let struct_name = parsed.name.to_string();

    // Generate field definitions
    let field_definitions = match &parsed.body {
        Some(body) => body
            .content
            .0
            .iter()
            .map(|field| {
                let field_name = field.value.name.to_string();

                // Determine field flags
                let mut flags = "facet::FieldFlags::EMPTY";
                for attr in &field.value.attributes {
                    if let AttributeInner::Facet(attr) = &attr.body.content {
                        match &attr.inner.content {
                            FacetInner::Sensitive(_ksensitive) => {
                                flags = "facet::FieldFlags::SENSITIVE"
                            }
                            FacetInner::Other(_) => {
                                // nothing
                            }
                        }
                        // Since FacetInner only has Sensitive variant, we can directly set flags
                    }
                }

                let mut attributes = vec![];
                for attr in &field.value.attributes {
                    if let AttributeInner::Facet(attr) = &attr.body.content {
                        match &attr.inner.content {
                            FacetInner::Sensitive(_ksensitive) => {
                                attributes.push("facet::FieldAttribute::Sensitive".to_string());
                            }
                            FacetInner::Other(token_trees) => {
                                attributes.push(format!(
                                    r#"facet::FieldAttribute::Arbitrary({:?})"#,
                                    format!("{:?}", token_trees)
                                ));
                            }
                        }
                    }
                }
                let attributes = attributes.join(",");
                let maybe_field_doc = build_maybe_doc(&field.value.attributes);

                // Generate each field definition
                format!(
                    "facet::Field::builder()
                .name(\"{field_name}\")
                .shape(facet::shape_of(&|s: {struct_name}| s.{field_name}))
                .offset(::core::mem::offset_of!({struct_name}, {field_name}))
                .flags({flags})
                .attributes(&[{attributes}])
                {maybe_field_doc}
                .build()"
                )
            })
            .collect::<Vec<String>>()
            .join(
                ",
            ",
            ),
        None => String::new(),
    };

    let static_decl = generate_static_decl(&struct_name);
    let maybe_container_doc = build_maybe_doc(&parsed.attributes);

    // Generate the impl
    let output = format!(
        r#"
{static_decl}

#[automatically_derived]
unsafe impl facet::Facet for {struct_name} {{
    const SHAPE: &'static facet::Shape = &const {{
        static FIELDS: &[facet::Field] = &[
            {field_definitions}
        ];

        facet::Shape::builder()
            .id(facet::ConstTypeId::of::<{struct_name}>())
            .layout(core::alloc::Layout::new::<Self>())
            .vtable(facet::value_vtable!(
                {struct_name},
                |f, _opts| core::fmt::Write::write_str(f, "{struct_name}")
            ))
            .def(facet::Def::Struct(facet::StructDef::builder()
                .kind(facet::StructKind::Struct)
                .fields(FIELDS)
                .build()))
            {maybe_container_doc}
            .build()
    }};
}}
        "#
    );

    output.into_token_stream().into()
}
