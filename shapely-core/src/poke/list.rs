use crate::{ListVTable, Opaque, Shape, ValueVTable};

pub struct PokeList<'mem> {
    pub data: Opaque<'mem>,
    pub shape: Shape,
    pub vtable: ValueVTable,
    pub list_vtable: ListVTable,
}
