use facet_derive_emit::*;
use rust_format::{Formatter, RustFmt};

fn expand(input: &str) -> String {
    RustFmt::default()
        .format_tokens(facet_derive(input.parse().unwrap()))
        .unwrap()
}

#[test]
fn test_codegen() {
    insta::assert_snapshot!(expand(
        r#"
        #[derive(Facet)]
        struct FooBar;
        "#
    ));
}
