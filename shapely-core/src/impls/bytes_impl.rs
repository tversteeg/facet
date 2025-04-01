impl Shapely for Bytes {
    fn shape() -> Shape {
        Shape {
            name: |f, _opts| write!(f, "Bytes"),
            typeid: mini_typeid::of::<Self>(),
            layout: std::alloc::Layout::new::<Self>(),
            innards: Innards::Scalar(Scalar::Bytes),
            set_to_default: Some(|addr: *mut u8| unsafe {
                *(addr as *mut Bytes) = Bytes(Vec::new());
            }),
            drop_in_place: Some(|addr: *mut u8| unsafe {
                std::ptr::drop_in_place(addr as *mut Bytes);
            }),
        }
    }
}
