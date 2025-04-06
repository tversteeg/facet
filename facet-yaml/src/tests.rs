#[cfg(test)]
mod tests {
    use super::*;
    use facet_core::Facet;

    #[derive(Debug, Facet, PartialEq)]
    struct Person {
        name: String,
        age: u64,
    }

    #[test]
    fn test_deserialize_person() {
        let yaml = r#"
            name: Alice
            age: 30
        "#;

        let mut partial = Person::partial();
        from_yaml(&mut partial, yaml).expect("Failed to parse YAML");

        let person = partial.build::<Person>();
        assert_eq!(
            person,
            Person {
                name: "Alice".to_string(),
                age: 30
            }
        );
    }
}
