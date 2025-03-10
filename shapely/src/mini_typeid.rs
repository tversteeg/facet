// vendored straight from https://github.com/dtolnay/typeid — which is dual-licensed under
// MIT and Apache-2.0.
//
// We don't really need const type_id construction or older rustc support, so this is a minimal
// take on it.

use std::{any::TypeId, marker::PhantomData};

#[must_use]
#[inline(always)]
pub fn of<T>() -> TypeId
where
    T: ?Sized,
{
    trait NonStaticAny {
        fn get_type_id(&self) -> TypeId
        where
            Self: 'static;
    }

    impl<T: ?Sized> NonStaticAny for PhantomData<T> {
        #[inline(always)]
        fn get_type_id(&self) -> TypeId
        where
            Self: 'static,
        {
            TypeId::of::<T>()
        }
    }

    let phantom_data = PhantomData::<T>;
    NonStaticAny::get_type_id(unsafe {
        std::mem::transmute::<&dyn NonStaticAny, &(dyn NonStaticAny + 'static)>(&phantom_data)
    })
}
