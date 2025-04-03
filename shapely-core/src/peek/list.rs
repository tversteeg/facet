use crate::{ListDef, OpaqueConst, Shape};

use super::PeekValue;

/// Lets you read from a value (implements read-only [`ListVTable`] proxies)
#[derive(Clone, Copy)]
pub struct PeekList<'mem> {
    pub(crate) data: OpaqueConst<'mem>,
    pub(crate) shape: &'static Shape,
    #[expect(dead_code)]
    pub(crate) def: ListDef,
}

impl<'mem> PeekList<'mem> {
    /// Coerce to a value
    pub fn as_value(self) -> PeekValue<'mem> {
        PeekValue {
            data: self.data,
            shape: self.shape,
        }
    }
}
