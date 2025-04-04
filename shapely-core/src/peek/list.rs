use crate::ListDef;

use super::PeekValue;

/// Lets you read from a list (implements read-only [`ListVTable`] proxies)
#[derive(Clone, Copy)]
pub struct PeekList<'mem> {
    value: PeekValue<'mem>,
    #[expect(dead_code)]
    def: ListDef,
}

impl<'mem> std::ops::Deref for PeekList<'mem> {
    type Target = PeekValue<'mem>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'mem> PeekList<'mem> {
    /// Creates a new peek list
    pub fn new(value: PeekValue<'mem>, def: ListDef) -> Self {
        Self { value, def }
    }
}
