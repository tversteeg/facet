use crate::Shapely;

#[test]
fn regular() {
    crate::parse_enum!(++.);
    crate::parse_enum!(+.);
    crate::parse_enum!(--.);
    crate::parse_enum!(-.);

    crate::parse_struct_like! {
        struct Yay {
            a: u32,
        }
    }
}

#[test]
fn special_cases() {
    #[derive(Shapely)]
    #[allow(dead_code)]
    struct Blah {
        foo: u32,
        bar: String,
    }

    println!("{}", Blah::get_parsed_structure())
}
