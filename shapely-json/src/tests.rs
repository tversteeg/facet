use super::*;

use shapely::Shapely;

#[test]
fn test_from_json() {
    #[derive(Debug, PartialEq)]
    struct TestStruct {
        name: String,
        age: u64,
    }

    impl Shapely for TestStruct {
        fn shape() -> shapely::Shape {
            use shapely::Innards;
            static SCHEMA: shapely::Shape = shapely::Shape {
                name: "TestStruct",
                layout: std::alloc::Layout::new::<TestStruct>(),
                innards: Innards::Struct {
                    fields: shapely::struct_fields!(TestStruct, (name, age)),
                },
                display: None,
                debug: Some(|addr: *const u8, f: &mut std::fmt::Formatter| {
                    std::fmt::Debug::fmt(unsafe { &*(addr as *const TestStruct) }, f)
                }),
                set_to_default: None,
                drop_in_place: Some(|ptr| unsafe {
                    std::ptr::drop_in_place(ptr as *mut TestStruct)
                }),
            };
            SCHEMA
        }
    }

    let json = r#"{"name": "Alice", "age": 30}"#;

    let mut test_struct = TestStruct::partial();
    let result = from_json(&mut test_struct, json);

    result.unwrap();
    let built_struct = test_struct.build::<TestStruct>();

    assert_eq!(
        built_struct,
        TestStruct {
            name: "Alice".to_string(),
            age: 30
        }
    );
}
