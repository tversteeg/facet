use facet::Facet;

#[derive(Debug, Facet, PartialEq)]
struct Person {
    name: String,
    age: u64,
}

#[test]
fn test_deserialize_person() {
    facet_testhelpers::setup();

    let toml = r#"
            name = "Alice"
            age = 30
        "#;

    let person: Person = facet_toml::from_str(toml).expect("Failed to parse TOML");
    assert_eq!(
        person,
        Person {
            name: "Alice".to_string(),
            age: 30
        }
    );
}
