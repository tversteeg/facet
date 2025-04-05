use owo_colors::OwoColorize;
use shapely_peek::Peek;

#[test]
fn test_sample_libc() {
    if !cfg!(miri) {
        let (data, shape) = shapely_samplelibc::get_foo_and_shape();
        let peek = unsafe { Peek::unchecked_new(data.as_const(), shape) };
        eprintln!("🔍 Display: {}", format!("{}", peek).bright_green());
        eprintln!("🐛 Debug: {}", format!("{:?}", peek).bright_blue());

        inspect(peek);
    }
}

fn inspect(peek: Peek) {
    inspect_with_indent(peek, 0);
}

fn inspect_with_indent(peek: Peek, indent: usize) {
    let indent_str = " ".repeat(indent * 4);

    let ps = match peek {
        Peek::Struct(ps) => ps,
        _ => {
            return;
        }
    };

    eprintln!(
        "{}📊 That struct has {} fields",
        indent_str,
        ps.field_count().to_string().bright_yellow()
    );
    for (k, v) in ps.fields() {
        eprintln!(
            "{}🔑 Field {} => {}",
            indent_str,
            k.to_string().bright_cyan(),
            v.to_string().bright_magenta()
        );
        inspect_with_indent(v.wrap(), indent + 1);
    }
}
