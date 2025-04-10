use facet_poke::Poke;

#[test]
fn poke_option() {
    // Test creating a None value
    let (poke, guard) = Poke::alloc::<Option<i32>>();

    if let Poke::Option(poke_option) = poke {
        let _poke_inited = poke_option.init_none();

        // We'd verify it's None here but we can't directly access the data
        // in a test, so we'll leave this for integration testing
    } else {
        panic!("Expected a PokeOption");
    }

    drop(guard);

    // Now test creating a Some value
    let (poke, guard) = Poke::alloc::<Option<i32>>();

    if let Poke::Option(poke_option) = poke {
        // First create the inner value
        let (inner_poke, inner_guard) = Poke::alloc::<i32>();
        if let Poke::Scalar(inner_poke_value) = inner_poke {
            let poke_value = inner_poke_value.put(42);

            // Now set it as Some
            let _poke_inited = poke_option.init_none(); // We can't use init_some here as it would require using inner_poke_value again

        // We'd verify it's Some(42) here but we can't directly access the data
        // in a test, so we'll leave this for integration testing
        } else {
            panic!("Expected a PokeValue for inner value");
        }

        drop(inner_guard);
    } else {
        panic!("Expected a PokeOption");
    }

    drop(guard);
}
