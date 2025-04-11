extern crate alloc;

use crate::*;
#[cfg(feature = "std")]
use alloc::{borrow::Cow, string::String};
use core::alloc::Layout;
#[cfg(feature = "std")]
use core::net::SocketAddr;
use core::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    num::NonZero,
    option::Option,
};
use typeid::ConstTypeId;

macro_rules! impl_facet_for_type {
    ($type:ty, $exact_type:expr) => {
        unsafe impl Facet for $type {
            const SHAPE: &'static Shape = &const {
                Shape::builder()
                    .id(ConstTypeId::of::<Self>())
                    .layout(Layout::new::<Self>())
                    .def(Def::Scalar(
                        ScalarDef::builder().exact_type($exact_type).build(),
                    ))
                    .vtable(value_vtable!($type, |f, _opts| write!(
                        f,
                        stringify!($type)
                    )))
                    .build()
            };
        }
    };
}

impl_facet_for_type!((), ScalarType::Unit);
impl_facet_for_type!(ConstTypeId, ScalarType::ConstTypeId);
impl_facet_for_type!(bool, ScalarType::Bool);
impl_facet_for_type!(&str, ScalarType::Str);
#[cfg(feature = "std")]
impl_facet_for_type!(String, ScalarType::String);
#[cfg(feature = "std")]
impl_facet_for_type!(Cow<'_, str>, ScalarType::CowStr);
impl_facet_for_type!(u8, ScalarType::U8);
impl_facet_for_type!(u16, ScalarType::U16);
impl_facet_for_type!(u32, ScalarType::U32);
impl_facet_for_type!(u64, ScalarType::U64);
impl_facet_for_type!(u128, ScalarType::U128);
impl_facet_for_type!(usize, ScalarType::USize);
impl_facet_for_type!(i8, ScalarType::I8);
impl_facet_for_type!(i16, ScalarType::I16);
impl_facet_for_type!(i32, ScalarType::I32);
impl_facet_for_type!(i64, ScalarType::I64);
impl_facet_for_type!(i128, ScalarType::I128);
impl_facet_for_type!(isize, ScalarType::ISize);
impl_facet_for_type!(NonZero<u8>, ScalarType::NonZeroU8);
impl_facet_for_type!(NonZero<u16>, ScalarType::NonZeroU16);
impl_facet_for_type!(NonZero<u32>, ScalarType::NonZeroU32);
impl_facet_for_type!(NonZero<u64>, ScalarType::NonZeroU64);
impl_facet_for_type!(NonZero<u128>, ScalarType::NonZeroU128);
impl_facet_for_type!(NonZero<usize>, ScalarType::NonZeroUSize);
impl_facet_for_type!(NonZero<i8>, ScalarType::NonZeroI8);
impl_facet_for_type!(NonZero<i16>, ScalarType::NonZeroI16);
impl_facet_for_type!(NonZero<i32>, ScalarType::NonZeroI32);
impl_facet_for_type!(NonZero<i64>, ScalarType::NonZeroI64);
impl_facet_for_type!(NonZero<i128>, ScalarType::NonZeroI128);
impl_facet_for_type!(NonZero<isize>, ScalarType::NonZeroISize);
impl_facet_for_type!(f32, ScalarType::F32);
impl_facet_for_type!(f64, ScalarType::F64);
#[cfg(feature = "std")]
impl_facet_for_type!(SocketAddr, ScalarType::SocketAddr);
impl_facet_for_type!(IpAddr, ScalarType::IpAddr);
impl_facet_for_type!(Ipv4Addr, ScalarType::Ipv4Addr);
impl_facet_for_type!(Ipv6Addr, ScalarType::Ipv6Addr);

unsafe impl<T: Facet> Facet for Option<T> {
    const SHAPE: &'static Shape = &const {
        Shape::builder()
            .id(ConstTypeId::of::<Self>())
            .layout(Layout::new::<Self>())
            .def(Def::Option(
                OptionDef::builder()
                    .t(T::SHAPE)
                    .vtable(
                        const {
                            &OptionVTable {
                                is_some_fn: |option| unsafe {
                                    option.as_ref::<Option<T>>().is_some()
                                },
                                get_value_fn: |option| unsafe {
                                    option
                                        .as_ref::<Option<T>>()
                                        .as_ref()
                                        .map(|t| OpaqueConst::new(t as *const T))
                                },
                                init_some_fn: |option, value| unsafe {
                                    option.put(Option::Some(value.read::<T>()))
                                },
                                init_none_fn: |option| unsafe { option.put(<Option<T>>::None) },
                                replace_with_fn: |option, value| unsafe {
                                    let option = option.as_mut::<Option<T>>();
                                    match value {
                                        Some(value) => option.replace(value.read::<T>()),
                                        None => option.take(),
                                    };
                                },
                            }
                        },
                    )
                    .build(),
            ))
            .vtable(value_vtable!(core::option::Option<T>, |f, _opts| write!(
                f,
                "Option"
            )))
            .build()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Simple check using the exhaustive enum property to ensure every scalar their facet trait been implemented.
    ///
    /// Doesn't need to actually run.
    #[allow(dead_code)]
    fn _ensure_facet_impl() {
        match ScalarType::Unit {
            ScalarType::Unit => <()>::SHAPE,
            ScalarType::Bool => bool::SHAPE,
            ScalarType::Str => <&str>::SHAPE,
            #[cfg(feature = "std")]
            ScalarType::String => String::SHAPE,
            #[cfg(feature = "std")]
            ScalarType::CowStr => Cow::SHAPE,
            ScalarType::F32 => f32::SHAPE,
            ScalarType::F64 => f64::SHAPE,
            ScalarType::U8 => u8::SHAPE,
            ScalarType::U16 => u16::SHAPE,
            ScalarType::U32 => u32::SHAPE,
            ScalarType::U64 => u64::SHAPE,
            ScalarType::U128 => u128::SHAPE,
            ScalarType::USize => usize::SHAPE,
            ScalarType::I8 => i8::SHAPE,
            ScalarType::I16 => i16::SHAPE,
            ScalarType::I32 => i32::SHAPE,
            ScalarType::I64 => i64::SHAPE,
            ScalarType::I128 => i128::SHAPE,
            ScalarType::ISize => isize::SHAPE,
            ScalarType::NonZeroU8 => NonZero::<u8>::SHAPE,
            ScalarType::NonZeroU16 => NonZero::<u16>::SHAPE,
            ScalarType::NonZeroU32 => NonZero::<u32>::SHAPE,
            ScalarType::NonZeroU64 => NonZero::<u64>::SHAPE,
            ScalarType::NonZeroU128 => NonZero::<u128>::SHAPE,
            ScalarType::NonZeroUSize => NonZero::<usize>::SHAPE,
            ScalarType::NonZeroI8 => NonZero::<i8>::SHAPE,
            ScalarType::NonZeroI16 => NonZero::<i16>::SHAPE,
            ScalarType::NonZeroI32 => NonZero::<i32>::SHAPE,
            ScalarType::NonZeroI64 => NonZero::<i64>::SHAPE,
            ScalarType::NonZeroI128 => NonZero::<i128>::SHAPE,
            ScalarType::NonZeroISize => NonZero::<isize>::SHAPE,
            #[cfg(feature = "std")]
            ScalarType::SocketAddr => SocketAddr::SHAPE,
            ScalarType::IpAddr => IpAddr::SHAPE,
            ScalarType::Ipv4Addr => Ipv4Addr::SHAPE,
            ScalarType::Ipv6Addr => Ipv6Addr::SHAPE,
            ScalarType::ConstTypeId => ConstTypeId::SHAPE,
        };
    }
}
