use facet_poke::Poke;
use facet_trait::Facet;
use facet_trait::ShapeExt;

pub fn from_slice<T: Facet>(s: &[&str]) -> T {
    let mut s = s;
    let (poke, guard) = Poke::alloc::<T>();
    let mut ps = poke.into_struct();

    loop {
        let token = s.first().unwrap();
        s = &s[1..];

        if let Some(key) = token.strip_prefix("--") {
            let (_field_index, field) = ps.field_by_name(key).unwrap();
            let value = s.first().unwrap();
            s = &s[1..];

            let field_shape = field.shape();
            let parse = field_shape
                .vtable
                .parse
                .unwrap_or_else(|| panic!("shape {} does not support parse", field.shape()));
            unsafe { (parse)(value, field.into_value().data()) }.unwrap_or_else(|e| {
                panic!(
                    "Failed to parse field '{}' of shape {}: {}",
                    key, field_shape, e
                )
            });
        } else {
            // Handle positional argument
        }
    }
}

#[cfg(test)]
#[test]
fn test_arg_parse() {
    use facet::Facet;

    #[derive(Facet)]
    struct Args {
        #[facet(positional)]
        path: String,

        #[facet(named, short = 'v')]
        verbose: bool,

        #[facet(named, short = 'j')]
        concurrency: usize,
    }

    let args: Args = crate::from_slice(&["--path", "example.rs", "--verbose", "--concurrency"]);
}
