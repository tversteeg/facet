//! Tests for TOML values to different forms of options.

use facet::Facet;

#[test]
fn test_option_scalar() {
    facet_testhelpers::setup();

    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        value: Option<i32>,
    }

    assert_eq!(
        facet_toml::from_str::<Root>("").expect("Failed to parse TOML"),
        Root { value: None },
    );
    assert_eq!(
        facet_toml::from_str::<Root>("value = 1").expect("Failed to parse TOML"),
        Root { value: Some(1) },
    );
}

#[test]
fn test_nested_option() {
    facet_testhelpers::setup();

    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        value: Option<Option<i32>>,
    }

    assert_eq!(
        facet_toml::from_str::<Root>("").expect("Failed to parse TOML"),
        Root { value: None },
    );
    assert_eq!(
        facet_toml::from_str::<Root>("value = 1").expect("Failed to parse TOML"),
        Root {
            value: Some(Some(1))
        },
    );
}

#[test]
fn test_option_struct() {
    facet_testhelpers::setup();

    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        value: Option<Item>,
    }

    #[derive(Debug, Facet, PartialEq)]
    struct Item {
        value: i32,
    }

    assert_eq!(
        facet_toml::from_str::<Root>("").expect("Failed to parse TOML"),
        Root { value: None },
    );
    assert_eq!(
        facet_toml::from_str::<Root>("value.value = 1").expect("Failed to parse TOML"),
        Root {
            value: Some(Item { value: 1 })
        },
    );
}

#[test]
fn test_option_struct_with_option() {
    facet_testhelpers::setup();

    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        value: Option<Item>,
    }

    #[derive(Debug, Facet, PartialEq)]
    struct Item {
        sub: Option<i32>,
    }

    assert_eq!(
        facet_toml::from_str::<Root>("").expect("Failed to parse TOML"),
        Root { value: None },
    );
    assert_eq!(
        facet_toml::from_str::<Root>("value.sub = 1").expect("Failed to parse TOML"),
        Root {
            value: Some(Item { sub: Some(1) })
        },
    );
}

#[test]
fn test_option_enum() {
    facet_testhelpers::setup();

    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        value: Option<Item>,
    }

    #[derive(Debug, Facet, PartialEq)]
    #[repr(u8)]
    enum Item {
        A,
        B(i32),
    }

    assert_eq!(
        facet_toml::from_str::<Root>("").expect("Failed to parse TOML"),
        Root { value: None },
    );
    assert_eq!(
        facet_toml::from_str::<Root>("value = 'A'").expect("Failed to parse TOML"),
        Root {
            value: Some(Item::A)
        },
    );
    assert_eq!(
        facet_toml::from_str::<Root>("value.B = 1").expect("Failed to parse TOML"),
        Root {
            value: Some(Item::B(1))
        },
    );
}

#[test]
fn test_option_enum_with_option_variant() {
    facet_testhelpers::setup();

    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        value: Option<Item>,
    }

    #[derive(Debug, Facet, PartialEq)]
    #[repr(u8)]
    enum Item {
        A,
        B(Option<i32>),
    }

    assert_eq!(
        facet_toml::from_str::<Root>("").expect("Failed to parse TOML"),
        Root { value: None },
    );
    assert_eq!(
        facet_toml::from_str::<Root>("value = 'A'").expect("Failed to parse TOML"),
        Root {
            value: Some(Item::A)
        },
    );
    assert_eq!(
        facet_toml::from_str::<Root>("value.B = 1").expect("Failed to parse TOML"),
        Root {
            value: Some(Item::B(Some(1)))
        },
    );
    // TODO: fix
    // assert_eq!(
    //     facet_toml::from_str::<Root>("value = 'B'").expect("Failed to parse TOML"),
    //     Root {
    //         value: Some(Item::B(None))
    //     },
    // );
}
