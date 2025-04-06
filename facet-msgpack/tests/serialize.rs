use ctor::ctor;

#[ctor]
fn init_backtrace() {
    color_backtrace::install();
}

use facet_derive::Facet;
use facet_msgpack::to_vec;
use facet_trait::Facet;

use facet_trait as facet;

#[test]
fn test_serialize() {
    #[derive(Debug, PartialEq, Clone, Facet)]
    struct TestStruct {
        name: String,
        age: u64,
    }

    let test_struct = TestStruct {
        name: "Alice".to_string(),
        age: 30,
    };

    let msgpack = to_vec(&test_struct);

    // Expected MessagePack format:
    // 82                -- map with 2 elements
    // a4 6e616d65      -- "name" (length 4)
    // a5 416c696365    -- "Alice" (length 5)
    // a3 616765        -- "age" (length 3)
    // ce 0000001e      -- 30 as uint32
    let expected = [
        0x82, 0xa4, 0x6e, 0x61, 0x6d, 0x65, 0xa5, 0x41, 0x6c, 0x69, 0x63, 0x65, 0xa3, 0x61, 0x67,
        0x65, 0xce, 0x00, 0x00, 0x00, 0x1e,
    ];

    assert_eq!(msgpack, expected);
}
