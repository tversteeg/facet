/// A type-erased read-only pointer to a value
pub struct OpaqueConst(pub *const u8);

/// A type-erased read-only reference to an initialized value
pub struct OpaqueConstRef<'a>(pub &'a u8);

/// A type-erased pointer to an uninitialized value
pub struct OpaqueUninit(pub *mut u8);

/// A type-erased pointer to an initialized value
pub struct Opaque(pub *mut u8);

/// A type-erased mutable reference to an initialized value
pub struct OpaqueRef<'a>(pub &'a mut u8);
