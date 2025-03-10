
use nonmax::NonMaxU32;

use crate::{Innards, Shape, Shapely};

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
                    offset: Some(
                        NonMaxU32::new(std::mem::offset_of!($struct, $field).try_into().unwrap())
                            .expect("your struct is larger than 4GiB? that's impressive"),
                    ),
                }
            };
        }
        static FOO_FIELD: MapField = map_field!(FooBar, foo);
        static BAR_FIELD: MapField = map_field!(FooBar, bar);

        static SCHEMA: Shape = Shape {
            name: "FooBar",
            size: std::mem::size_of::<FooBar>(),
            align: std::mem::align_of::<FooBar>(),
            innards: Innards::Map(
                MapInnards::builder()
                    .field(FOO_FIELD)
                    .field(BAR_FIELD)
                    .open_ended(false)
                    .mk_slots(|shape| &StructManipulator { shape })
                    .build(),
            ),
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
        let foo_bar = unsafe { &mut *(ptr as *mut FooBar) };
        for field in sh.static_fields() {
            unsafe {
                match field.name {
                    "foo" => {
                        if let Some(slot) = sh.slots.slot(foo_bar, *field) {
                            slot.fill(42u64);
                        }
                    }
                    "bar" => {
                        if let Some(slot) = sh.slots.slot(foo_bar, *field) {
                            slot.fill(String::from("Hello, World!"));
                        }
                    }
                    _ => panic!("Unknown field: {}", field.name),
                }
            }
        }
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
