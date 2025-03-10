use insta::assert_debug_snapshot;
use shapely::Shapely;

#[test]
fn simple_struct() {
    #[derive(Debug, Shapely)]
    struct Blah {
        foo: u32,
        bar: String,
    }

    assert_debug_snapshot!(Blah::shape());
}
