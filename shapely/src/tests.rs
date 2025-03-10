use nonmax::NonMaxU32;

use crate::{Innards, Shape, ShapeUninit, Shapely};

#[derive(Debug, PartialEq, Eq)]
struct FooBar {
    foo: u64,
    bar: String,
}

impl Shapely for FooBar {
    fn shape() -> crate::Shape {
        use crate::{Innards, MapField, MapInnards};

        fn shape_of<TStruct, TField: Shapely>(_f: impl Fn(TStruct) -> TField) -> Shape {
            TField::shape()
        }
        macro_rules! map_field {
            ($struct:ty, $field:ident) => {
                MapField {
                    name: stringify!($field),
                    schema: || shape_of(|s: $struct| s.$field),
                    offset: Some({
                        let offset = std::mem::offset_of!($struct, $field);
                        if offset > u32::MAX as usize {
                            panic!("Struct field offset exceeds u32::MAX");
                        }
                        NonMaxU32::new(offset as u32)
                            .expect("Field offset should never be u32::MAX")
                    }),
                }
            };
        }
        static FOO_FIELD: MapField = map_field!(FooBar, foo);
        static BAR_FIELD: MapField = map_field!(FooBar, bar);

        static SCHEMA: Shape = Shape {
            name: "FooBar",
            size: std::mem::size_of::<FooBar>(),
            align: std::mem::align_of::<FooBar>(),
            innards: Innards::Map(MapInnards::for_struct(&[FOO_FIELD, BAR_FIELD])),
            display: None,
            debug: None,
            set_to_default: None,
        };
        SCHEMA
    }
}

#[test]
fn build_foobar_through_reflection() {
    let schema = FooBar::shape();

    let layout = std::alloc::Layout::from_size_align(schema.size, schema.align).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) };
    if ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }

    if let Innards::Map(sh) = &schema.innards {
        let mut uninit = FooBar::shape_uninit();
        for field in sh.static_fields() {
            let slot = uninit.slot(field).unwrap();
            match field.name {
                "foo" => {
                    slot.fill(42u64);
                }
                "bar" => {
                    slot.fill(String::from("Hello, World!"));
                }
                _ => panic!("Unknown field: {}", field.name),
            }
        }
        let foo_bar = unsafe { &*(uninit.addr as *const FooBar) };
    }

    // Verify the fields were set correctly
    let foo_bar = unsafe { &*(ptr as *const FooBar) };
    assert_eq!(foo_bar.foo, 42);
    assert_eq!(foo_bar.bar, "Hello, World!");

    assert_eq!(
        &FooBar {
            foo: 42,
            bar: "Hello, World!".to_string()
        },
        foo_bar
    )
}
