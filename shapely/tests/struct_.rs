use shapely::{OpaqueConst, Poke, Shapely};

// Allow dead code in test modules since we're not constructing all enum variants
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Shapely)]
struct FooBar {
    foo: u64,
    bar: String,
}

#[test]
fn build_foobar_through_reflection() {
    let mut poke = Poke::alloc::<FooBar>().into_struct();
    poke.set_by_name("foo", OpaqueConst::from_ref(&42u64))
        .unwrap();

    {
        let bar = String::from("Hello, World!");
        poke.set_by_name("bar", OpaqueConst::from_ref(&bar))
            .unwrap();
        // bar has been moved out of
        std::mem::forget(bar);
    }

    let foo_bar = poke.build::<FooBar>();

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
