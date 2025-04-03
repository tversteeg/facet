use crate::peek::PeekValue;
use crate::{OpaqueConst, Shape, StructDef};
use std::fmt;

/// Lets you read from a struct (implements read-only struct operations)
#[derive(Clone, Copy)]
pub struct PeekStruct<'mem> {
    pub(crate) data: OpaqueConst<'mem>,
    pub(crate) shape: &'static Shape,
    // I suppose this could be a `&'static` as well, idk
    pub(crate) def: StructDef,
}

impl<'mem> PeekStruct<'mem> {
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
            let field_data = self.data.field(field.offset);
            PeekValue {
                data: field_data,
                shape: field.shape,
            }
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

    /// Coerce to a value
    pub fn as_value(self) -> PeekValue<'mem> {
        PeekValue {
            data: self.data,
            shape: self.shape,
        }
    }
}

// Helper struct for field debug formatting
struct DebugFieldWrapper<'a, 'mem>(&'a PeekValue<'mem>);

impl<'a, 'mem> fmt::Debug for DebugFieldWrapper<'a, 'mem> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(result) = self.0.debug(f) {
            result
        } else {
            write!(f, "<unprintable>")
        }
    }
}
