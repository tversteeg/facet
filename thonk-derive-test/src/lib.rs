// this crate exists only for tests

#[cfg(test)]
mod tests {
    use thonk_derive::Thonk;

    #[derive(Thonk)]
    struct Blah {}
}
