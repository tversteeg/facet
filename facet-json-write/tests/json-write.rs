use facet_core::Facet;
use facet_derive::Facet;
use facet_json_write::to_json;
use facet_poke::Peek;

use facet_core as facet;

#[ctor::ctor]
fn init() {
    // Initialize color backtrace for pretty stack traces
    color_backtrace::install();

    // Initialize logger to print all log levels
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace"))
        .format_timestamp(None)
        .init();

    log::info!("Logging and color backtrace initialized");
}

#[test]
fn test_to_json() {
    #[derive(Debug, PartialEq, Clone, Facet)]
    struct TestStruct {
        name: String,
        age: u64,
    }

    let test_struct = TestStruct {
        name: "Alice".to_string(),
        age: 30,
    };

    let _expected_json = r#"{"name":"Alice","age":30}"#;
    let expected_json_indented = r#"{
  "name": "Alice",
  "age": 30
}"#;

    let mut buffer = Vec::new();
    let peek = Peek::new(&test_struct);
    to_json(peek, &mut buffer, true).unwrap();
    let json = String::from_utf8(buffer).unwrap();
    assert_eq!(json, expected_json_indented);
}
