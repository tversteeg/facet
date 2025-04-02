use shapely::{OpaqueConst, Poke, Shapely};

#[test]
fn build_u64_properly() {
    let shape = u64::shape();
    eprintln!("{shape:#?}");

    let scalar = Poke::alloc::<u64>().into_scalar();
    let data = unsafe { scalar.put(OpaqueConst::from_ref(&42u64)) };
    let value = unsafe { data.read::<u64>() };

    // Verify the value was set correctly
    assert_eq!(value, 42);
}

#[test]
#[should_panic(expected = "Field '0' was not initialized")]
fn build_u64_unfilled() {
    use crate::{Poke, Shapely};

    let shape = u64::shape();
    eprintln!("{shape:#?}");

    let poke = Poke::alloc::<u64>().into_scalar();

    // Intentionally not filling the value
    let _value = poke
        .into_scalar()
        .default_in_place()
        .expect("Failed to set default");
    // This should panic
}
