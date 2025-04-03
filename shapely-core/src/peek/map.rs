use crate::{MapDef, OpaqueConst, Shape};

use super::PeekValue;

/// Lets you read from a value (implements read-only [`MapVTable`] proxies)
#[derive(Clone, Copy)]
pub struct PeekMap<'mem> {
    pub(crate) data: OpaqueConst<'mem>,
    pub(crate) shape: &'static Shape,
    #[expect(dead_code)]
    pub(crate) def: MapDef,
}

impl<'mem> PeekMap<'mem> {
    /// Coerce to a value
    pub fn as_value(self) -> PeekValue<'mem> {
        PeekValue {
            data: self.data,
            shape: self.shape,
        }
    }
}
