use crate::{Shape, ShapeDesc, Shapely};

#[doc(hidden)]
pub fn shape_of<TStruct, TField: Shapely>(_f: impl Fn(TStruct) -> TField) -> Shape {
    TField::shape()
}

#[doc(hidden)]
pub const fn shape_desc_of<TStruct, TField: Shapely>(_f: &dyn Fn(TStruct) -> TField) -> ShapeDesc {
    ShapeDesc(TField::shape)
}

#[doc(hidden)]
#[macro_export]
macro_rules! struct_field {
    ($struct:ty, $field:ident) => {
        $crate::Field {
            name: stringify!($field),
            shape: $crate::shape_desc_of(&|s: $struct| s.$field),
            offset: ::std::mem::offset_of!($struct, $field),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! struct_fields {
    ($struct:ty, ($($field:ident),*)) => {{
        static FIELDS: &[$crate::Field] = &[ $($crate::struct_field!($struct, $field)),* ];
        FIELDS
    }};
}
