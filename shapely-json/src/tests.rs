use super::*;

use shapely::Shapely;

#[test]
fn test_from_json() {
    #[derive(Shapely)]
    struct TestStruct {
        name: String,
        age: u64,
    }
    let json = r#"{"name": "Alice", "age": 30}"#;

    let mut test_struct = TestStruct::partial();
    from_json(&mut test_struct, json).unwrap();

    let built_struct = test_struct.build::<TestStruct>();
    assert_eq!(built_struct.name, "Alice");
    assert_eq!(built_struct.age, 30);
}

#[test]
fn test_to_json() {
    #[derive(Debug, PartialEq)]
    struct TestStruct {
        name: String,
        age: u64,
    }

    impl Shapely for TestStruct {
        fn shape() -> shapely::Shape {
            shapely::Shape {
                name: |f| write!(f, "TestStruct"),
                layout: std::alloc::Layout::new::<Self>(),
                innards: shapely::Innards::Struct {
                    fields: shapely::struct_fields!(TestStruct, (name, age)),
                },
                set_to_default: None,
                drop_in_place: Some(|ptr| unsafe { std::ptr::drop_in_place(ptr as *mut Self) }),
                typeid: std::any::TypeId::of::<Self>(),
            }
        }
    }

    let test_struct = TestStruct {
        name: "Alice".to_string(),
        age: 30,
    };

    let expected_json = r#"{"name":"Alice","age":30}"#;
    let expected_json_indented = r#"{
  "name": "Alice",
  "age": 30
}"#;

    let mut buffer = Vec::new();
    to_json(
        &test_struct as *const _ as *mut u8,
        TestStruct::shape_desc(),
        &mut buffer,
        false,
    )
    .unwrap();
    let json = String::from_utf8(buffer).unwrap();
    assert_eq!(json, expected_json);

    let mut buffer = Vec::new();
    to_json(
        &test_struct as *const _ as *mut u8,
        TestStruct::shape_desc(),
        &mut buffer,
        true,
    )
    .unwrap();
    let json_indented = String::from_utf8(buffer).unwrap();
    assert_eq!(json_indented, expected_json_indented.trim());

    // Test roundtrip
    let mut deserialized = TestStruct::partial();
    from_json(&mut deserialized, expected_json).unwrap();
    let deserialized_struct = deserialized.build::<TestStruct>();
    assert_eq!(deserialized_struct, test_struct);
}

#[test]
fn test_from_json_with_more_types() {
    #[derive(Shapely)]
    struct TestStructWithMoreTypes {
        u8_val: u8,
        u16_val: u16,
        i8_val: i8,
        i16_val: i16,
        u32_val: u32,
        i32_val: i32,
        u64_val: u64,
        i64_val: i64,
        f32_val: f32,
        f64_val: f64,
    }

    let json = r#"{
        "u8_val": 255,
        "u16_val": 65535,
        "i8_val": -128,
        "i16_val": -32768,
        "u32_val": 4294967295,
        "i32_val": -2147483648,
        "u64_val": 18446744073709551615,
        "i64_val": -9223372036854775808,
        "f32_val": 3.14,
        "f64_val": 3.141592653589793
    }"#;

    let mut test_struct = TestStructWithMoreTypes::partial();
    from_json(&mut test_struct, json).unwrap();

    let built_struct = test_struct.build::<TestStructWithMoreTypes>();
    assert_eq!(built_struct.u8_val, 255);
    assert_eq!(built_struct.u16_val, 65535);
    assert_eq!(built_struct.i8_val, -128);
    assert_eq!(built_struct.i16_val, -32768);
    assert_eq!(built_struct.u32_val, 4294967295);
    assert_eq!(built_struct.i32_val, -2147483648);
    assert_eq!(built_struct.u64_val, 18446744073709551615);
    assert_eq!(built_struct.i64_val, -9223372036854775808);
    assert!((built_struct.f32_val - 3.14).abs() < f32::EPSILON);
    assert!((built_struct.f64_val - 3.141592653589793).abs() < f64::EPSILON);
}

#[test]
fn test_from_json_with_nested_structs() {
    #[derive(Shapely)]
    struct InnerStruct {
        value: i32,
    }

    #[derive(Shapely)]
    struct OuterStruct {
        name: String,
        inner: InnerStruct,
    }

    let json = r#"{
        "name": "Outer",
        "inner": {
            "value": 42
        }
    }"#;

    let mut test_struct = OuterStruct::partial();
    from_json(&mut test_struct, json).unwrap();

    let built_struct = test_struct.build::<OuterStruct>();
    assert_eq!(built_struct.name, "Outer");
    assert_eq!(built_struct.inner.value, 42);
}

// #[test]
// fn test_from_json_with_tuples() {
//     #[derive(Shapely)]
//     struct TupleStruct(i32, String, (f64, bool));

//     let json = r#"[123, "Hello", [3.14, true]]"#;

//     let mut test_struct = TupleStruct::partial();
//     from_json(&mut test_struct, json).unwrap();

//     let built_struct = test_struct.build::<TupleStruct>();
//     assert_eq!(built_struct.0, 123);
//     assert_eq!(built_struct.1, "Hello");
//     assert!((built_struct.2.0 - 3.14).abs() < f64::EPSILON);
//     assert_eq!(built_struct.2.1, true);
// }
