use shapely::Shapely;

#[test]
fn simple_struct() {
    #[derive(Debug, Shapely)]
    #[allow(dead_code)]
    struct Blah {
        foo: u32,
        bar: String,
    }

    eprintln!("{:?}", Blah::shape())
}
