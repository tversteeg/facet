// this crate exists only for tests

#[cfg(test)]
mod tests {
    use shapely::Shapely;

    #[derive(Shapely)]
    struct Blah {
        a: u32,
    }

    #[test]
    fn regular() {
        shapely::parse_enum!(++.);
        shapely::parse_enum!(+.);
        shapely::parse_enum!(--.);
        shapely::parse_enum!(-.);
        shapely::parse_enum!(,$);

        shapely::parse_struct_like! {
            struct Yay {
                a: u32,
            }
        }
    }
}
