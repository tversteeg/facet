use facet_trait::MapDef;

use super::PeekValue;

/// Lets you read from a map (implements read-only [`MapVTable`] proxies)
#[derive(Clone, Copy)]
pub struct PeekMap<'mem> {
    value: PeekValue<'mem>,
    #[expect(dead_code)]
    def: MapDef,
}

impl<'mem> std::ops::Deref for PeekMap<'mem> {
    type Target = PeekValue<'mem>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'mem> PeekMap<'mem> {
    /// Constructor
    pub fn new(value: PeekValue<'mem>, def: MapDef) -> Self {
        Self { value, def }
    }
}
