use shapely::{OpaqueConst, Poke, Shapely};

#[test]
fn build_u64_properly() {
    let shape = u64::SHAPE;
    eprintln!("{shape:#?}");

    let (poke, _guard) = Poke::alloc::<u64>();
    let poke = poke.into_scalar();
    let data = unsafe { poke.put(OpaqueConst::from_ref(&42u64)) };
    let value = unsafe { data.read::<u64>() };

    // Verify the value was set correctly
    assert_eq!(value, 42);
}
