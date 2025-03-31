use shapely::Shapely;
use std::mem::offset_of;

#[test]
fn simple_struct() {
    #[derive(Debug, Shapely)]
    struct Blah {
        foo: u32,
        bar: String,
    }

    if !cfg!(miri) {
        let shape = Blah::shape();

        // Check the name using Display
        assert_eq!(format!("{}", shape), "Blah");

        assert_eq!(shape.layout.size(), 32);
        assert_eq!(shape.layout.align(), 8);

        if let shapely::Innards::Struct { fields } = shape.innards {
            assert_eq!(fields.len(), 2);

            let foo_field = &fields[0];
            assert_eq!(foo_field.name, "foo");
            assert_eq!(foo_field.shape.get().layout.size(), 4);
            assert_eq!(foo_field.shape.get().layout.align(), 4);
            assert_eq!(foo_field.offset, offset_of!(Blah, foo));

            let bar_field = &fields[1];
            assert_eq!(bar_field.name, "bar");
            assert_eq!(bar_field.shape.get().layout.size(), 24);
            assert_eq!(bar_field.shape.get().layout.align(), 8);
            assert_eq!(bar_field.offset, offset_of!(Blah, bar));
        } else {
            panic!("Expected Struct innards");
        }
    }
}

#[test]
fn struct_repr_c() {
    #[derive(Clone, Hash, PartialEq, Eq, ::shapely::Shapely)]
    #[repr(C)]
    struct Blah {
        foo: u32,
        bar: String,
    }
}

#[test]
fn tuple_struct_repr_transparent() {
    #[derive(Clone, Hash, PartialEq, Eq, ::shapely::Shapely)]
    #[repr(transparent)]
    struct Blah(u32);
}

#[test]
fn tuple_struct_doc_comment() {
    #[derive(Clone, Hash, PartialEq, Eq, ::shapely::Shapely)]
    #[repr(transparent)]
    /// This is a struct for sure
    struct Blah(u32);
}

#[test]
fn tuple_struct_field_doc_comment() {
    #[derive(Clone, Hash, PartialEq, Eq, ::shapely::Shapely)]
    #[repr(transparent)]
    /// This is a struct for sure
    struct Blah(
        /// and this is a field
        u32,
    );
}

#[test]
fn tuple_struct_with_pub_field() {
    #[derive(Clone, Hash, PartialEq, Eq, ::shapely::Shapely)]
    #[repr(transparent)]
    /// This is a struct for sure
    struct Blah(
        /// and this is a field
        pub u32,
    );
}

// #[test]
// fn struct_with_generic() {
//     #[derive(Debug, ::shapely::Shapely)]
//     struct Generic<T> {
//         data: T,
//     }

//     let shape = Generic::<u32>::shape();
//     assert_eq!(format!("{}", shape), "Generic<u32>");
// }

// #[test]
// fn struct_with_lifetime() {
//     #[derive(Debug, ::shapely::Shapely)]
//     struct WithLifetime<'a> {
//         reference: &'a str,
//     }

//     let shape = WithLifetime::shape();
//     assert_eq!(format!("{}", shape), "WithLifetime");
// }

// #[test]
// fn tuple_struct() {
//     #[derive(Debug, ::shapely::Shapely)]
//     struct Point(f32, f32);

//     let shape = Point::shape();
//     assert_eq!(format!("{}", shape), "Point");
//     if let shapely::Innards::Struct { fields } = shape.innards {
//         assert_eq!(fields.len(), 2);
//         assert_eq!(fields[0].name, "0");
//         assert_eq!(fields[1].name, "1");
//     } else {
//         panic!("Expected Struct innards");
//     }
// }

// #[test]
// fn unit_struct() {
//     /// A unit struct with documentation
//     #[derive(Debug, ::shapely::Shapely)]
//     struct Unit;

//     let shape = Unit::shape();
//     assert_eq!(format!("{}", shape), "Unit");
//     assert!(matches!(shape.innards, shapely::Innards::Struct { fields } if fields.is_empty()));
// }

// #[test]
// fn struct_with_attributes() {
//     #[derive(Debug, ::shapely::Shapely)]
//     #[repr(C, packed)]
//     struct Packed {
//         a: u8,
//         b: u32,
//     }

//     let shape = Packed::shape();
//     assert_eq!(shape.layout.size(), 5);
//     assert_eq!(shape.layout.align(), 1);
// }

// #[test]
// fn enum_test() {
//     #[derive(Debug, ::shapely::Shapely)]
//     enum MyEnum {
//         A,
//         B(i32),
//         C { x: f64, y: f64 },
//     }

//     let shape = MyEnum::shape();
//     assert_eq!(format!("{}", shape), "MyEnum");
//     if let shapely::Innards::Enum { variants, .. } = shape.innards {
//         assert_eq!(variants.len(), 3);
//         assert_eq!(variants[0].name, "A");
//         assert_eq!(variants[1].name, "B");
//         assert_eq!(variants[2].name, "C");
//     } else {
//         panic!("Expected Enum innards");
//     }
// }
