use facet_trait::StructDef;

use crate::PeekValue;

/// Lets you read from a struct (implements read-only struct operations)
#[derive(Clone, Copy)]
pub struct PeekStruct<'mem> {
    value: PeekValue<'mem>,
    // I suppose this could be a `&'static` as well, idk
    def: StructDef,
}

impl<'mem> std::ops::Deref for PeekStruct<'mem> {
    type Target = PeekValue<'mem>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'mem> PeekStruct<'mem> {
    /// Create a new peek struct
    pub(crate) fn new(value: PeekValue<'mem>, def: StructDef) -> Self {
        Self { value, def }
    }

    /// Returns the number of fields in this struct
    #[inline(always)]
    pub fn field_count(&self) -> usize {
        self.def.fields.len()
    }

    /// Returns the name of the field at the given index
    #[inline(always)]
    pub fn field_name(&self, index: usize) -> Option<&'static str> {
        self.def.fields.get(index).map(|field| field.name)
    }

    /// Returns the value of the field at the given index
    #[inline(always)]
    pub fn field_value(&self, index: usize) -> Option<PeekValue<'mem>> {
        self.def.fields.get(index).map(|field| unsafe {
            let field_data = self.data().field(field.offset);
            PeekValue::new(field_data, field.shape)
        })
    }

    /// Returns the value of the field with the given name
    #[inline(always)]
    pub fn get_field(&self, name: &str) -> Option<PeekValue<'mem>> {
        self.def
            .fields
            .iter()
            .position(|field| field.name == name)
            .and_then(|index| self.field_value(index))
    }

    /// Iterates over all fields in this struct, providing both name and value
    #[inline]
    pub fn fields(&self) -> impl Iterator<Item = (&'static str, PeekValue<'mem>)> + '_ {
        (0..self.field_count()).filter_map(|i| {
            let name = self.field_name(i)?;
            let value = self.field_value(i)?;
            Some((name, value))
        })
    }
}
