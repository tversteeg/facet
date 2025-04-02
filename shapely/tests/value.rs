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
