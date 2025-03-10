
use shapely::Shapely;

#[derive(Shapely)]
pub struct Blah {
    foo: u32,
    bar: String,
}

#[test]
fn regular() {
    shapely::parse_enum!(++.);
    shapely::parse_enum!(+.);
    shapely::parse_enum!(--.);
    shapely::parse_enum!(-.);

    shapely::parse_struct_like! {
        struct Yay {
            a: u32,
        }
    }

    println!("{}", Blah::get_parsed_structure())
}
