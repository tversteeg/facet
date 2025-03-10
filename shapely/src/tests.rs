use crate::{Shape, Shapely};

#[derive(Debug, PartialEq, Eq)]
struct FooBar {
    foo: u64,
    bar: String,
}

impl Shapely for FooBar {
    fn shape() -> crate::Shape {
        use crate::Innards;

        static SCHEMA: Shape = Shape {
            name: "FooBar",
            size: std::mem::size_of::<FooBar>(),
            align: std::mem::align_of::<FooBar>(),
            innards: Innards::Struct {
                fields: crate::struct_fields!(FooBar, (foo, bar)),
            },
            display: None,
            debug: Some(|addr: *const u8, f: &mut std::fmt::Formatter| {
                std::fmt::Debug::fmt(unsafe { &*(addr as *const FooBar) }, f)
            }),
            set_to_default: None,
        };
        SCHEMA
    }
}

#[test]
fn build_foobar_through_reflection() {
    let shape = FooBar::shape();
    eprintln!("{shape:#?}");

    let layout = std::alloc::Layout::from_size_align(shape.size, shape.align).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }

    let mut uninit = FooBar::shape_uninit();
    for field in shape.innards.static_fields() {
        let slot = uninit.slot(*field).unwrap();
        match field.name {
            "foo" => {
                slot.fill(42u64);
            }
            "bar" => {
                slot.fill(String::from("Hello, World!"));
            }
            _ => panic!("Unknown field: {}", field.name),
        }
    }
    let foo_bar = uninit.build::<FooBar>();

    // Verify the fields were set correctly
    assert_eq!(foo_bar.foo, 42);
    assert_eq!(foo_bar.bar, "Hello, World!");

    assert_eq!(
        FooBar {
            foo: 42,
            bar: "Hello, World!".to_string()
        },
        foo_bar
    )
}

#[test]
fn build_u64_through_reflection() {
    let shape = u64::shape();
    eprintln!("{shape:#?}");

    let layout = std::alloc::Layout::from_size_align(shape.size, shape.align).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }

    let mut uninit = u64::shape_uninit();
    let slot = uninit.scalar_slot().unwrap();
    slot.fill(42u64);
    let value = uninit.build::<u64>();

    // Verify the value was set correctly
    assert_eq!(value, 42);
}

#[test]
#[should_panic(expected = "attempted to build uninitialised slot")]
fn build_u64_through_reflection_without_filling() {
    let shape = u64::shape();
    eprintln!("{shape:#?}");

    let layout = std::alloc::Layout::from_size_align(shape.size, shape.align).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }

    let uninit = u64::shape_uninit();
    // Intentionally not filling the slot
    let _value = uninit.build::<u64>();
    // This should panic
}

#[test]
#[should_panic(expected = "We were building a \u{1b}[1;33mu64\u{1b}[0m")]
fn build_u64_get_u32_through_reflection() {
    let shape = u64::shape();
    eprintln!("{shape:#?}");

    let layout = std::alloc::Layout::from_size_align(shape.size, shape.align).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }

    let mut uninit = u64::shape_uninit();
    let slot = uninit.scalar_slot().unwrap();
    slot.fill(42u64);

    // Attempt to build as u32 instead of u64
    let _value = uninit.build::<u32>();
    // This should panic due to type mismatch
}
