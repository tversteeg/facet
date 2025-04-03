use shapely::{OpaqueConst, OpaqueUninit, Poke, Shapely};

// Allow dead code in test modules since we're not constructing all enum variants
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Shapely)]
struct FooBar {
    foo: u64,
    bar: String,
}

impl Default for FooBar {
    fn default() -> Self {
        FooBar {
            foo: 69,
            bar: String::new(),
        }
    }
}

#[test]
fn build_foobar_through_reflection() {
    eprintln!("At this point, FooBar ShapeId is {:?}", FooBar::shape_id());

    eprintln!("Allocating FooBar");
    let (poke, guard) = Poke::alloc::<FooBar>();
    eprintln!("Converting poke to struct");
    let mut poke = poke.into_struct();
    eprintln!("Setting 'foo' field");
    poke.set_by_name("foo", OpaqueConst::from_ref(&42u64))
        .unwrap();
    eprintln!("Creating 'bar' string");

    {
        eprintln!("Creating 'bar' string");
        let bar = String::from("Hello, World!");
        eprintln!("Setting 'bar' field");
        poke.set_by_name("bar", OpaqueConst::from_ref(&bar))
            .unwrap();
        eprintln!("Forgetting 'bar'");
        // bar has been moved out of
        std::mem::forget(bar);
    }

    eprintln!("Building FooBar");
    eprintln!("At this point, FooBar ShapeId is {:?}", FooBar::shape_id());
    let foo_bar = poke.build::<FooBar>(Some(guard));

    eprintln!("Verifying fields");
    // Verify the fields were set correctly
    assert_eq!(foo_bar.foo, 42);
    eprintln!("Verified 'foo' field");
    assert_eq!(foo_bar.bar, "Hello, World!");
    eprintln!("Verified 'bar' field");

    eprintln!("Final assertion");
    assert_eq!(
        FooBar {
            foo: 42,
            bar: "Hello, World!".to_string()
        },
        foo_bar
    )
}

#[test]
#[should_panic(expected = "Field 'bar' was not initialized")]
fn build_foobar_incomplete() {
    let (poke, guard) = Poke::alloc::<FooBar>();
    let mut poke = poke.into_struct();
    poke.set_by_name("foo", OpaqueConst::from_ref(&42u64))
        .unwrap();

    let foo_bar = poke.build::<FooBar>(Some(guard));

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
fn build_foobar_after_default() {
    let mut foo_bar: FooBar = Default::default();

    let mut poke = unsafe {
        Poke::from_opaque_uninit(OpaqueUninit::new(&mut foo_bar as *mut _), FooBar::SHAPE)
    }
    .into_struct();
    unsafe {
        poke.mark_initialized(0);
        poke.mark_initialized(1);
    }

    {
        let bar = String::from("Hello, World!");
        poke.set_by_name("bar", OpaqueConst::from_ref(&bar))
            .unwrap();
        // bar has been moved out of
        std::mem::forget(bar);
    }
    poke.build_in_place();

    // Verify the fields were set correctly
    assert_eq!(foo_bar.foo, 69);
    assert_eq!(foo_bar.bar, "Hello, World!");
}
