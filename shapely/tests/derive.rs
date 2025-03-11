use std::mem::offset_of;

use nonmax::NonMaxU32;
use shapely::Shapely;

#[test]
fn simple_struct() {
    #[derive(Debug, Shapely)]
    struct Blah {
        foo: u32,
        bar: String,
    }

    if !cfg!(miri) {
        let shape = Blah::shape();
        assert_eq!(shape.name, "Blah");
        assert_eq!(shape.layout.size(), 32);
        assert_eq!(shape.layout.align(), 8);

        if let shapely::Innards::Struct { fields } = shape.innards {
            assert_eq!(fields.len(), 2);

            let foo_field = &fields[0];
            assert_eq!(foo_field.name, "foo");
            assert_eq!(foo_field.shape.get().layout.size(), 4);
            assert_eq!(foo_field.shape.get().layout.align(), 4);
            assert_eq!(
                foo_field.offset,
                Some(NonMaxU32::new(offset_of!(Blah, foo) as u32).unwrap())
            );

            let bar_field = &fields[1];
            assert_eq!(bar_field.name, "bar");
            assert_eq!(bar_field.shape.get().layout.size(), 24);
            assert_eq!(bar_field.shape.get().layout.align(), 8);
            assert_eq!(
                bar_field.offset,
                Some(NonMaxU32::new(offset_of!(Blah, bar) as u32).unwrap())
            );
        } else {
            panic!("Expected Struct innards");
        }
    }
}
