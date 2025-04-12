//! Tests for TOML values to structs.

use facet::Facet;

#[test]
fn test_unit_only_enum() {
    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        value: UnitOnlyEnum,
    }

    #[derive(Debug, Facet, PartialEq)]
    #[repr(u8)]
    enum UnitOnlyEnum {
        VariantA,
        VariantB,
    }

    assert_eq!(
        facet_toml::from_str::<Root>("value = 'VariantA'").expect("Failed to parse TOML"),
        Root {
            value: UnitOnlyEnum::VariantA,
        },
    );
    assert_eq!(
        facet_toml::from_str::<Root>("value = 'VariantB'").expect("Failed to parse TOML"),
        Root {
            value: UnitOnlyEnum::VariantB,
        },
    );
}

#[test]
fn test_single_value_on_non_unit_enum() {
    #[derive(Debug, Facet, PartialEq)]
    struct Root {
        value: WithNonUnitVariant,
    }

    #[derive(Debug, Facet, PartialEq)]
    #[repr(u8)]
    enum WithNonUnitVariant {
        VariantA,
        #[allow(dead_code)]
        VariantB(i32),
    }

    assert_eq!(
        facet_toml::from_str::<Root>("value = 'VariantA'").expect("Failed to parse TOML"),
        Root {
            value: WithNonUnitVariant::VariantA
        },
    );
    assert!(facet_toml::from_str::<Root>("value = 'VariantB'").is_err());
}
