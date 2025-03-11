use super::*;

use shapely_core::Shapely;
// this makes the derives work.. dirty but oh well.
use shapely_core as shapely;
use shapely_derive::Shapely;

#[test]
fn it_works() {
    #[derive(Debug, PartialEq, Shapely)]
    struct TestStruct {
        name: String,
        age: u64,
    }

    let data = [
        0x82, // Fixmap with 2 elements
        0xa4, 0x6e, 0x61, 0x6d, 0x65, // Fixstr "name"
        0xa5, 0x41, 0x6c, 0x69, 0x63, 0x65, // Fixstr "Alice"
        0xa3, 0x61, 0x67, 0x65, // Fixstr "age"
        0x1a, 0x00, 0x00, 0x00, 0x1e, // uint32 30
    ];

    let mut partial = Partial::alloc(TestStruct::shape_desc());
    from_msgpack(&mut partial, &data).unwrap();

    let result = partial.build::<TestStruct>();
    assert_eq!(
        result,
        TestStruct {
            name: "Alice".to_string(),
            age: 30,
        }
    );
}
