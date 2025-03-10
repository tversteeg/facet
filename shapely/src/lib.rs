//! Provides the core traits for shapely

#[cfg(feature = "derive")]
pub use shapely_derive::*;

pub use nonmax::NonMaxU32;

use std::alloc;

mod hashmap_impl;
mod scalar_impls;

mod shape;
pub use shape::*;

mod slot;
pub use slot::Slot;

mod uninit;
pub use uninit::*;

#[cfg(all(test, feature = "derive"))]
mod derive_tests;

/// Provides reflection so you can shapely about your types.
pub trait Shapely {
    /// Returns the shape of this type
    fn shape() -> Shape;

    fn uninit() -> ShapeUninit {
        let shape = Self::shape();
        let layout = alloc::Layout::from_size_align(shape.size, shape.align).unwrap();
        let addr = unsafe { alloc::alloc(layout) };
        if addr.is_null() {
            alloc::handle_alloc_error(layout);
        }
        ShapeUninit {
            addr,
            init_fields: InitFields64::new(),
            shape,
        }
    }
}

#[cfg(test)]
mod tests {
    use nonmax::NonMaxU32;

    use crate::{Innards, Shape, Shapely, StructManipulator};

    #[derive(Debug, PartialEq, Eq)]
    struct FooBar {
        foo: u64,
        bar: String,
    }

    impl Shapely for FooBar {
        fn shape() -> crate::Shape {
            use crate::{Innards, MapField, MapInnards};

            static FOO_FIELD: MapField = MapField {
                name: "foo",
                schema: <u64 as Shapely>::shape,
                offset: Some(NonMaxU32::new(std::mem::offset_of!(FooBar, foo) as u32).unwrap()),
            };
            static BAR_FIELD: MapField = MapField {
                name: "bar",
                schema: <String as Shapely>::shape,
                offset: Some(NonMaxU32::new(std::mem::offset_of!(FooBar, bar) as u32).unwrap()),
            };
            static SCHEMA: Shape = Shape {
                name: "FooBar",
                size: std::mem::size_of::<FooBar>(),
                align: std::mem::align_of::<FooBar>(),
                innards: Innards::Map(MapInnards {
                    fields: &[FOO_FIELD, BAR_FIELD],
                    open_ended: false,
                    slots: &StructManipulator { shape: SCHEMA },
                }),
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
            for field in sh.fields {
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
}
