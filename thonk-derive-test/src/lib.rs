// this crate exists only for tests

#[cfg(test)]
mod tests {
    // use thonk::Thonk;

    // #[derive(Thonk)]
    // struct Blah {}

    #[test]
    fn regular() {
        thonk::parse_enum!(++.);
        thonk::parse_enum!(+.);
        thonk::parse_enum!(--.);
        thonk::parse_enum!(-.);
        thonk::parse_enum!(,$);

        thonk::parse_struct_like! {
            // struct Yay {}
            struct Yay
        }
    }
}
