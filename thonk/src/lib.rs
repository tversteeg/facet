//! Provides the core traits for thonk

mod builtin_impls;

/// Provides reflection so you can thonk about your types.
pub trait Schematic {
    /// Returns the thonk schema
    fn schema() -> &'static Schema;
}

/// Schema for reflection of a type
pub struct Schema {
    /// A descriptive name for the schema, e.g. `u64`, or `Person`
    pub name: &'static str,

    /// Size of one such value, in bytes
    pub size: usize,

    /// Alignment of the value, in bytes
    pub align: usize,

    /// Shape of the value
    pub shape: Shape,

    /// Display impl, if any
    pub display: Option<FmtFunction>,

    /// Debug impl, if any
    pub debug: Option<FmtFunction>,

    /// Set the value at a given address to the default value
    pub set_to_default: Option<fn(*mut u8)>,
}

impl std::fmt::Debug for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Schema")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("align", &self.align)
            .field("shape", &self.shape)
            .field("write_display", &self.display.is_some())
            .field("write_debug", &self.debug.is_some())
            .finish()
    }
}

/// The shape of a schema: is it more map-shaped, array-shaped, scalar-shaped?
#[derive(Debug, Clone, Copy)]
pub enum Shape {
    /// Associates keys with values
    Map(MapShape),

    /// Ordered list of heterogenous values, variable size
    Array(&'static Schema),

    /// Ordered list of non-heterogenous values, fixed-size
    /// Tuple(&'static [&'static Schema]),

    /// Transparent — forwards to another known schema
    Transparent(&'static Schema),

    /// Scalar — known based type
    Scalar(Scalar),
}

/// The shape of a map: works for structs, but also HashMap<String, String> for example
#[derive(Clone, Copy)]
pub struct MapShape {
    /// Statically-known fields
    fields: &'static [MapField],

    /// Will allow setting fields outside of the ones listed in `fields`
    open_ended: bool,

    /// Setter for fields — we can't use field offsets
    setter: &'static dyn MapManipulator,
}

impl std::fmt::Debug for MapShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapShape")
            .field("fields", &self.fields)
            .field("open_ended", &self.open_ended)
            .finish()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MapField {
    /// key for the map field
    name: &'static str,

    /// schema of the inner type
    schema: &'static Schema,
}

/// Given the map's address, calls on_field_addr with the address of the requested field
pub trait MapManipulator: Send + Sync + 'static {
    unsafe fn set_field_raw(
        &self,
        map_addr: *mut u8,
        field: &MapField,
        on_addr: &dyn FnMut(*mut u8),
    );
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Scalar {
    // Valid utf-8
    String,

    // Not valid utf-8 🤷
    Bytes,

    I8,
    I16,
    I32,
    I64,
    I128,

    U8,
    U16,
    U32,
    U64,
    U128,

    F32,
    F64,

    Boolean,
}

/// A function that writes a field to a formatter
pub type FmtFunction = fn(addr: *const u8, &mut std::fmt::Formatter) -> std::fmt::Result;

#[cfg(test)]
mod tests {}
