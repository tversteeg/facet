use facet_poke::PokeUninit;

#[test]
fn poke_option() {
    // Test creating a None value
    let (poke, guard) = PokeUninit::alloc::<Option<i32>>();
    let po = poke.into_option();
    let po = unsafe { po.init_none() };
    let option: Option<i32> = po.build(Some(guard));
    assert_eq!(option, None);

    // Now test creating a Some value
    let (poke, guard) = PokeUninit::alloc::<Option<i32>>();
    let po = poke.into_option();
    let po = unsafe { po.put(42) };
    let option: Option<i32> = po.build(Some(guard));
    assert_eq!(option, Some(42));
}
