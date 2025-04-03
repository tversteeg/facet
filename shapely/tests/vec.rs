use shapely::Peek;

#[test]
fn vec_can_be_debug_or_not() {
    let v: Vec<i32> = vec![1, 2, 3];
    let peek = Peek::new(&v);
    println!("{peek:#?}");

    // #[derive(Shapely)]
    // struct NotDebug {
    //     blah: i32,
    // }
    // let v = vec![NotDebug { blah: 42 }];
    // let shape = Shape::of_val(&v);
    // assert!(shape.vtable.debug.is_none());
}
