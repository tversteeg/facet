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
    eprintln!("Address of test_struct: {:p}", test_struct.addr());
    let result = from_json(&mut test_struct, json);
    result.unwrap();

    let shape = TestStruct::shape();
    let age_field = shape.innards.static_fields()[1];
    let name_field = shape.innards.static_fields()[0];

    let age_addr = unsafe {
        test_struct
            .addr()
            .byte_offset(age_field.offset.unwrap().get() as isize)
    };
    let name_addr = unsafe {
        test_struct
            .addr()
            .byte_offset(name_field.offset.unwrap().get() as isize)
    };

    eprintln!("Age address: \x1b[33m{:p}\x1b[0m", age_addr);
    eprintln!("Name address: \x1b[33m{:p}\x1b[0m", name_addr);

    let age_value = unsafe { *(age_addr as *const u64) };
    let name_value = unsafe { &*(name_addr as *const String) };

    eprintln!("Age value before build: \x1b[33m{}\x1b[0m", age_value);
    eprintln!("Name value before build: \x1b[33m{}\x1b[0m", name_value);
    eprintln!(
        "Name pointer before build: \x1b[33m{:p}\x1b[0m",
        name_value.as_ptr()
    );
    eprintln!(
        "Name length before build: \x1b[33m{}\x1b[0m",
        name_value.len()
    );

    let built_struct = test_struct.build::<TestStruct>();
    eprintln!(
        "built_struct age address = \x1b[33m{:p}\x1b[0m",
        &built_struct.age as *const u64
    );
    eprintln!(
        "built_struct name address = \x1b[33m{:p}\x1b[0m",
        &built_struct.name as *const String
    );
    eprintln!("built_struct age = \x1b[33m{}\x1b[0m", built_struct.age);
    eprintln!(
        "built_struct name ptr = \x1b[33m{:p}\x1b[0m",
        built_struct.name.as_ptr()
    );
    eprintln!(
        "built_struct name len = \x1b[33m{}\x1b[0m",
        built_struct.name.len()
    );

    let built_struct_ptr = &built_struct as *const TestStruct as *const u8;
    let built_struct_size = std::mem::size_of::<TestStruct>();
    let built_struct_slice =
        unsafe { std::slice::from_raw_parts(built_struct_ptr, built_struct_size) };
    eprintln!("built_struct as hex:");
    for (i, byte) in built_struct_slice.iter().enumerate() {
        eprint!("{:02x} ", byte);
        if (i + 1) % 16 == 0 {
            eprintln!();
        }
    }
    eprintln!();

    assert_eq!(
        built_struct,
        TestStruct {
            name: "Alice".to_string(),
            age: 30
        }
    );
}
