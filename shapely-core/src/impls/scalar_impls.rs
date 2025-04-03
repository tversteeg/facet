use crate::*;
use std::alloc::Layout;

impl Shapely for () {
    const SHAPE: &'static Shape = &const {
        Shape {
            layout: Layout::new::<Self>(),
            def: Def::Scalar(ScalarDef::of::<Self>()),
            vtable: &ValueVTable {
                type_name: |f, _opts| write!(f, "()"),
                display: Some(|_value, f| write!(f, "()")),
                debug: Some(|_value, f| write!(f, "()")),
                default_in_place: Some(|target| unsafe {
                    Some({
                        Self::default();
                        target.write(())
                    })
                }),
                eq: Some(|_left, _right| true), // () == () is always true
                cmp: Some(|_left, _right| std::cmp::Ordering::Equal), // () cmp () is always Equal
                hash: Some(|_value, _hasher_self, _hasher_write_fn| {}),
                drop_in_place: None, // unit type doesn't need dropping
                parse: Some(|s, target| {
                    if s == "()" {
                        Some(unsafe { target.write(()) })
                    } else {
                        None
                    }
                }),
                try_from: None,
            },
        }
    };
}

impl Shapely for String {
    const SHAPE: &'static Shape = &Shape {
        layout: Layout::new::<Self>(),
        def: Def::Scalar(ScalarDef::of::<Self>()),
        vtable: &ValueVTable {
            type_name: |f, _opts| write!(f, "String"),
            display: display_fn_for::<Self>(),
            debug: debug_fn_for::<Self>(),
            default_in_place: Some(|target| unsafe { Some(target.write(Self::default())) }),
            eq: Some(|left, right| unsafe { left.as_ref::<Self>() == right.as_ref::<Self>() }),
            cmp: Some(|left, right| unsafe { left.as_ref::<Self>().cmp(right.as_ref::<Self>()) }),
            hash: Some(|value, hasher_self, hasher_write_fn| unsafe {
                use std::hash::Hash;
                value
                    .as_ref::<Self>()
                    .hash(&mut HasherProxy::new(hasher_self, hasher_write_fn));
            }),
            drop_in_place: Some(|value| unsafe {
                std::ptr::drop_in_place(value.as_mut_ptr::<Self>());
            }),
            parse: Some(|s, target| Some(unsafe { target.write(s.to_string()) })),
            try_from: None,
        },
    };
}

impl Shapely for bool {
    const SHAPE: &'static Shape = &Shape {
        layout: Layout::new::<Self>(),
        def: Def::Scalar(ScalarDef::of::<Self>()),
        vtable: &ValueVTable {
            type_name: |f, _opts| write!(f, "bool"),
            display: Some(|value, f| {
                let val = unsafe { value.as_ref::<Self>() };
                write!(f, "{val}")
            }),
            debug: debug_fn_for::<Self>(),
            default_in_place: Some(|target| unsafe { Some(target.write(Self::default())) }),
            eq: Some(|left, right| unsafe { left.as_ref::<Self>() == right.as_ref::<Self>() }),
            cmp: Some(|left, right| unsafe { left.as_ref::<Self>().cmp(right.as_ref::<Self>()) }),
            hash: Some(|value, hasher_self, hasher_write_fn| unsafe {
                use std::hash::Hash;
                value
                    .as_ref::<Self>()
                    .hash(&mut HasherProxy::new(hasher_self, hasher_write_fn));
            }),
            drop_in_place: None, // bool doesn't need dropping
            parse: Some(|s, target| {
                s.parse::<Self>()
                    .ok()
                    .map(|value| unsafe { target.write(value) })
            }),
            try_from: None,
        },
    };
}

macro_rules! impl_shapely_for_integer {
    ($type:ty) => {
        impl Shapely for $type {
            const SHAPE: &'static Shape = &Shape {
                layout: Layout::new::<Self>(),
                def: Def::Scalar(ScalarDef::of::<Self>()),
                vtable: &ValueVTable {
                    type_name: |f, _opts| write!(f, stringify!($type)),
                    display: display_fn_for::<Self>(),
                    debug: debug_fn_for::<Self>(),
                    default_in_place: Some(|target| unsafe { Some(target.write(Self::default())) }),
                    eq: Some(|left, right| unsafe {
                        left.as_ref::<Self>() == right.as_ref::<Self>()
                    }),
                    cmp: Some(|left, right| unsafe {
                        left.as_ref::<Self>().cmp(right.as_ref::<Self>())
                    }),
                    hash: Some(|value, hasher_self, hasher_write_fn| unsafe {
                        use std::hash::Hash;
                        value
                            .as_ref::<Self>()
                            .hash(&mut HasherProxy::new(hasher_self, hasher_write_fn));
                    }),
                    drop_in_place: None,
                    parse: Some(|s, target| {
                        s.parse::<Self>()
                            .ok()
                            .map(|value| unsafe { target.write(value) })
                    }),
                    try_from: None,
                },
            };
        }
    };
}

impl_shapely_for_integer!(u8);
impl_shapely_for_integer!(i8);
impl_shapely_for_integer!(u16);
impl_shapely_for_integer!(i16);
impl_shapely_for_integer!(u32);
impl_shapely_for_integer!(i32);
impl_shapely_for_integer!(u64);
impl_shapely_for_integer!(i64);
impl_shapely_for_integer!(u128);
impl_shapely_for_integer!(i128);

macro_rules! impl_shapely_for_float {
    ($type:ty) => {
        impl Shapely for $type {
            const SHAPE: &'static Shape = &Shape {
                layout: Layout::new::<Self>(),
                def: Def::Scalar(ScalarDef::of::<Self>()),
                vtable: &ValueVTable {
                    type_name: |f, _opts| write!(f, stringify!($type)),
                    display: Some(|value, f| {
                        let val = unsafe { *value.as_ptr::<Self>() };
                        write!(f, "{val}")
                    }),
                    debug: debug_fn_for::<Self>(),
                    default_in_place: Some(|target| unsafe { Some(target.write(Self::default())) }),
                    eq: Some(|left, right| unsafe {
                        left.as_ref::<Self>() == right.as_ref::<Self>()
                    }),
                    cmp: Some(|left, right| unsafe {
                        left.as_ref::<Self>()
                            .partial_cmp(right.as_ref::<Self>())
                            .unwrap_or(std::cmp::Ordering::Equal)
                    }),
                    hash: Some(|value, hasher_self, hasher_write_fn| unsafe {
                        use std::hash::Hash;
                        value
                            .as_ref::<Self>()
                            .to_bits()
                            .hash(&mut HasherProxy::new(hasher_self, hasher_write_fn));
                    }),
                    drop_in_place: None,
                    parse: Some(|s, target| {
                        s.parse::<Self>()
                            .ok()
                            .map(|value| unsafe { target.write(value) })
                    }),
                    try_from: None,
                },
            };
        }
    };
}

impl_shapely_for_float!(f32);
impl_shapely_for_float!(f64);
