//! Tests for TOML optional values.

// use facet::Facet;

// #[test]
// fn test_option_with_scalar() {
//     #[derive(Debug, Facet, PartialEq)]
//     struct Root {
//         value: Option<i32>,
//     }

//     assert_eq!(
//         facet_toml::from_str::<Root>("value = 1").expect("Failed to parse TOML"),
//         Root { value: Some(1) },
//     );

//     assert_eq!(
//         facet_toml::from_str::<Root>("").expect("Failed to parse TOML"),
//         Root { value: None },
//     );
// }

// #[test]
// fn test_option_with_unit_struct() {
//     #[derive(Debug, Facet, PartialEq)]
//     struct Root {
//         value: Option<Unit>,
//     }

//     #[derive(Debug, Facet, PartialEq)]
//     struct Unit(i32);

//     assert_eq!(
//         facet_toml::from_str::<Root>("value = 1").expect("Failed to parse TOML"),
//         Root {
//             value: Some(Unit(1))
//         },
//     );

//     assert_eq!(
//         facet_toml::from_str::<Root>("").expect("Failed to parse TOML"),
//         Root { value: None },
//     );
// }
