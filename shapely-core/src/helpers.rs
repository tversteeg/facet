use crate::{Shape, Shapely};

#[doc(hidden)]
pub const fn shape_of<TStruct, TField: Shapely>(_f: &dyn Fn(TStruct) -> TField) -> &'static Shape {
    TField::SHAPE
}

#[doc(hidden)]
#[macro_export]
macro_rules! struct_field {
    ($struct:ty, $field:tt) => {
        $crate::Field {
            name: stringify!($field),
            shape: $crate::shape_of(&|s: $struct| s.$field),
            offset: ::std::mem::offset_of!($struct, $field),
            flags: $crate::FieldFlags::EMPTY,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! struct_fields {
    ($struct:ty, ($($field:tt),*)) => {{
        static FIELDS: &[$crate::Field] = &[ $($crate::struct_field!($struct, $field)),* ];
        FIELDS
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! enum_unit_variant {
    ($enum:ty, $variant:ident) => {
        $crate::Variant {
            name: stringify!($variant),
            discriminant: None,
            kind: $crate::VariantKind::Unit,
        }
    };
    ($enum:ty, $variant:ident, $discriminant:expr) => {
        $crate::Variant {
            name: stringify!($variant),
            discriminant: Some($discriminant),
            kind: $crate::VariantKind::Unit,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! enum_tuple_variant {
    ($enum:ty, $variant:ident, [$($field_type:ty),*]) => {{
        static FIELDS: &[$crate::Field] = &[
            $(
                $crate::Field {
                    name: concat!("_", stringify!($field_type)),
                    shape_fn: <$field_type>::shape,
                    offset: 0, // Will be calculated at runtime
                    flags: $crate::FieldFlags::EMPTY,
                }
            ),*
        ];

        $crate::Variant {
            name: stringify!($variant),
            discriminant: None,
            kind: $crate::VariantKind::Tuple { fields: FIELDS },
        }
    }};
    ($enum:ty, $variant:ident, [$($field_type:ty),*], $discriminant:expr) => {{
        static FIELDS: &[$crate::Field] = &[
            $(
                $crate::Field {
                    name: concat!("_", stringify!($field_type)),
                    shape_fn: <$field_type>::shape,
                    offset: 0, // Will be calculated at runtime
                    flags: $crate::FieldFlags::EMPTY,
                }
            ),*
        ];

        $crate::Variant {
            name: stringify!($variant),
            discriminant: Some($discriminant),
            kind: $crate::VariantKind::Tuple { fields: FIELDS },
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! enum_struct_variant {
    ($enum:ty, $variant:ident, {$($field:ident: $field_type:ty),*}) => {{
        static FIELDS: &[$crate::Field] = &[
            $(
                $crate::Field {
                    name: stringify!($field),
                    shape_fn: <$field_type>::shape,
                    offset: 0, // Will be calculated at runtime
                    flags: $crate::FieldFlags::EMPTY,
                }
            ),*
        ];

        $crate::Variant {
            name: stringify!($variant),
            discriminant: None,
            kind: $crate::VariantKind::Struct { fields: FIELDS },
        }
    }};
    ($enum:ty, $variant:ident, {$($field:ident: $field_type:ty),*}, $discriminant:expr) => {{
        static FIELDS: &[$crate::Field] = &[
            $(
                $crate::Field {
                    name: stringify!($field),
                    shape_fn: <$field_type>::shape,
                    offset: 0, // Will be calculated at runtime
                    flags: $crate::FieldFlags::EMPTY,
                }
            ),*
        ];

        $crate::Variant {
            name: stringify!($variant),
            discriminant: Some($discriminant),
            kind: $crate::VariantKind::Struct { fields: FIELDS },
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! enum_variants {
    ($enum:ty, [$($variant:expr),*]) => {{
        static VARIANTS: &[$crate::Variant] = &[ $($variant),* ];
        VARIANTS
    }};
}
