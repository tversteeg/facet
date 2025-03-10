use crate::{Shape, Shapely};

#[derive(Debug, PartialEq, Eq)]
struct FooBar {
    foo: u64,
    bar: String,
}

impl Shapely for FooBar {
    fn shape() -> crate::Shape {
        static SHAPE: Shape = Shape {
            name: "FooBar",
            size: std::mem::size_of::<FooBar>(),
            align: std::mem::align_of::<FooBar>(),
            innards: crate::Innards::Struct {
                fields: crate::struct_fields!(FooBar, (foo, bar)),
            },
            display: None,
            debug: Some(|addr: *const u8, f: &mut std::fmt::Formatter| {
                std::fmt::Debug::fmt(unsafe { &*(addr as *const FooBar) }, f)
            }),
            set_to_default: None,
        };
        SHAPE
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
#[should_panic(expected = "Scalar value was not initialized")]
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
#[should_panic(expected = "Field 'bar' was not initialized")]
fn build_foobar_through_reflection_with_missing_field() {
    let shape = FooBar::shape();
    eprintln!("{shape:#?}");

    let layout = std::alloc::Layout::from_size_align(shape.size, shape.align).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }

    let mut uninit = FooBar::shape_uninit();
    for field in shape.innards.static_fields() {
        if field.name == "foo" {
            let slot = uninit.slot(*field).unwrap();
            slot.fill(42u64);
            // Intentionally not setting the 'bar' field
        }
    }

    // This should panic because 'bar' is not initialized
    let _foo_bar = uninit.build::<FooBar>();
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

#[test]
fn build_struct_with_drop_field() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

    struct DropCounter;

    impl Shapely for DropCounter {
        fn shape() -> crate::Shape {
            Shape {
                name: "DropCounter",
                size: std::mem::size_of::<DropCounter>(),
                align: std::mem::align_of::<DropCounter>(),
                innards: crate::Innards::Struct { fields: &[] },
                display: None,
                debug: None,
                set_to_default: None,
            }
        }
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            DROP_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    struct StructWithDrop {
        counter: DropCounter,
        value: i32,
    }

    impl Shapely for StructWithDrop {
        fn shape() -> crate::Shape {
            static SHAPE: Shape = Shape {
                name: "StructWithDrop",
                size: std::mem::size_of::<StructWithDrop>(),
                align: std::mem::align_of::<StructWithDrop>(),
                innards: crate::Innards::Struct {
                    fields: crate::struct_fields!(StructWithDrop, (counter, value)),
                },
                display: None,
                debug: None,
                set_to_default: None,
            };
            SHAPE
        }
    }

    let shape = StructWithDrop::shape();
    let mut uninit = StructWithDrop::shape_uninit();

    let counter_field = shape.innards.static_fields()[0];
    let value_field = shape.innards.static_fields()[1];

    // First assignment
    {
        let slot = uninit.slot(counter_field).unwrap();
        slot.fill(DropCounter);
    }

    // Second assignment, should trigger drop of the first value
    {
        let slot = uninit.slot(counter_field).unwrap();
        slot.fill(DropCounter);
    }

    // Set the value field
    {
        let slot = uninit.slot(value_field).unwrap();
        slot.fill(42i32);
    }

    let _struct_with_drop = uninit.build::<StructWithDrop>();

    // Check that drop was called once for the first assignment
    assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 1);

    // Explicitly drop _struct_with_drop
    drop(_struct_with_drop);

    // Check that drop was called twice: once for the first assignment and once for the final instance
    assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn build_scalar_with_drop() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

    struct DropScalar;

    impl Shapely for DropScalar {
        fn shape() -> crate::Shape {
            Shape {
                name: "DropScalar",
                size: std::mem::size_of::<DropScalar>(),
                align: std::mem::align_of::<DropScalar>(),
                innards: crate::Innards::Scalar(crate::Scalar::Nothing),
                display: None,
                debug: None,
                set_to_default: None,
            }
        }
    }

    impl Drop for DropScalar {
        fn drop(&mut self) {
            DROP_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    let mut uninit = DropScalar::shape_uninit();

    // First assignment
    {
        let slot = uninit.scalar_slot().unwrap();
        slot.fill(DropScalar);
    }

    // Second assignment, should trigger drop of the first value
    {
        let slot = uninit.scalar_slot().unwrap();
        slot.fill(DropScalar);
    }

    let _drop_scalar = uninit.build::<DropScalar>();

    // Check that drop was called once for the first assignment
    assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 1);

    // Explicitly drop _drop_scalar
    drop(_drop_scalar);

    // Check that drop was called twice: once for the first assignment and once for the final instance
    assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 2);
}

#[test]
fn build_truck_with_drop_fields() {
    use std::sync::atomic::{AtomicIsize, Ordering};

    static ENGINE_COUNT: AtomicIsize = AtomicIsize::new(0);
    static WHEELS_COUNT: AtomicIsize = AtomicIsize::new(0);

    struct Engine;
    struct Wheels;

    impl Drop for Engine {
        fn drop(&mut self) {
            ENGINE_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl Drop for Wheels {
        fn drop(&mut self) {
            WHEELS_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl Shapely for Engine {
        fn shape() -> crate::Shape {
            Shape {
                name: "Engine",
                size: std::mem::size_of::<Engine>(),
                align: std::mem::align_of::<Engine>(),
                innards: crate::Innards::Scalar(crate::Scalar::Nothing),
                display: None,
                debug: None,
                set_to_default: None,
            }
        }
    }

    impl Shapely for Wheels {
        fn shape() -> crate::Shape {
            Shape {
                name: "Wheels",
                size: std::mem::size_of::<Wheels>(),
                align: std::mem::align_of::<Wheels>(),
                innards: crate::Innards::Scalar(crate::Scalar::Nothing),
                display: None,
                debug: None,
                set_to_default: None,
            }
        }
    }

    struct Truck {
        engine: Engine,
        wheels: Wheels,
    }

    impl Shapely for Truck {
        fn shape() -> crate::Shape {
            static SHAPE: Shape = Shape {
                name: "Truck",
                size: std::mem::size_of::<Truck>(),
                align: std::mem::align_of::<Truck>(),
                innards: crate::Innards::Struct {
                    fields: crate::struct_fields!(Truck, (engine, wheels)),
                },
                display: None,
                debug: None,
                set_to_default: None,
            };
            SHAPE
        }
    }

    let shape = Truck::shape();
    let engine_field = shape.innards.static_fields()[0];
    let wheels_field = shape.innards.static_fields()[1];

    fn reset_atomics() {
        ENGINE_COUNT.store(0, Ordering::SeqCst);
        WHEELS_COUNT.store(0, Ordering::SeqCst);
    }

    // Scenario 1: Not filling any fields
    {
        reset_atomics();
        let uninit = Truck::shape_uninit();
        drop(uninit);
        assert_eq!(ENGINE_COUNT.load(Ordering::SeqCst), 0, "No drops occurred.");
        assert_eq!(WHEELS_COUNT.load(Ordering::SeqCst), 0, "No drops occurred.");
    }

    // Scenario 2: Filling only the engine field
    {
        reset_atomics();
        let mut uninit = Truck::shape_uninit();
        {
            let slot = uninit.slot(engine_field).unwrap();
            slot.fill(Engine);
        }
        drop(uninit);
        assert_eq!(
            ENGINE_COUNT.load(Ordering::SeqCst),
            1,
            "Engine field should have been dropped."
        );
        assert_eq!(
            WHEELS_COUNT.load(Ordering::SeqCst),
            0,
            "Wheels field wasn't set."
        );
    }

    // Scenario 3: Filling only the wheels field
    {
        reset_atomics();
        let mut uninit = Truck::shape_uninit();
        {
            let slot = uninit.slot(wheels_field).unwrap();
            slot.fill(Wheels);
        }
        drop(uninit);
        assert_eq!(
            ENGINE_COUNT.load(Ordering::SeqCst),
            0,
            "Engine field wasn't set."
        );
        assert_eq!(
            WHEELS_COUNT.load(Ordering::SeqCst),
            1,
            "Wheels field should have been dropped."
        );
    }

    // Scenario 4: Filling both fields
    {
        reset_atomics();
        let mut uninit = Truck::shape_uninit();
        {
            let slot = uninit.slot(engine_field).unwrap();
            slot.fill(Engine);
        }
        {
            let slot = uninit.slot(wheels_field).unwrap();
            slot.fill(Wheels);
        }
        drop(uninit);
        assert_eq!(
            ENGINE_COUNT.load(Ordering::SeqCst),
            1,
            "Engine field should have been dropped."
        );
        assert_eq!(
            WHEELS_COUNT.load(Ordering::SeqCst),
            1,
            "Wheels field should have been dropped."
        );
    }
}
