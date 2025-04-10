use facet_core::{EnumDef, EnumRepr, Variant};

/// Lets you read from an enum (implements read-only enum operations)
#[derive(Clone, Copy)]
pub struct PeekEnum<'mem> {
    value: crate::PeekValue<'mem>,
    def: EnumDef,
}

impl<'mem> core::ops::Deref for PeekEnum<'mem> {
    type Target = crate::PeekValue<'mem>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'mem> PeekEnum<'mem> {
    /// Create a new peek enum
    pub(crate) fn new(value: crate::PeekValue<'mem>, def: EnumDef) -> Self {
        Self { value, def }
    }

    /// Returns the enum definition
    #[inline(always)]
    pub fn def(&self) -> &EnumDef {
        &self.def
    }

    /// Returns the enum representation
    #[inline(always)]
    pub fn repr(&self) -> EnumRepr {
        self.def.repr
    }

    /// Returns the enum variants
    #[inline(always)]
    pub fn variants(&self) -> &'static [Variant] {
        self.def.variants
    }

    /// Returns the number of variants in this enum
    #[inline(always)]
    pub fn variant_count(&self) -> usize {
        self.def.variants.len()
    }

    /// Returns the variant name at the given index
    #[inline(always)]
    pub fn variant_name(&self, index: usize) -> Option<&'static str> {
        self.def.variants.get(index).map(|variant| variant.name)
    }

    /// Returns the variant index for this enum value
    #[inline]
    pub fn variant_index(&self) -> usize {
        // Use the discriminant function to get the variant index
        // Note: This assumes the enum has a "get_discriminant" function in the vtable
        // If not implemented yet, this would need to be added
        // For now, we'll return 0 as a placeholder
        0
    }

    /// Returns the name of the active variant for this enum value
    #[inline]
    pub fn variant_name_active(&self) -> &'static str {
        let index = self.variant_index();
        self.variant_name(index).expect("Invalid variant index")
    }
}

#[cfg(test)]
mod tests {

    // Skip the tests for now as they require facet_derive which is only a dev-dependency
    // and we haven't set that up properly yet
    /*
    use facet_derive::Facet;
    use facet_core as facet;

    #[derive(Facet)]
    #[repr(u8)]
    enum TestEnum {
        A,
        B(u32),
        C { x: i32, y: i32 },
    }*/

    /* Tests commented out until we have proper setup for facet_derive
    #[test]
    fn test_peek_enum() {
        let shape = TestEnum::SHAPE;
        let enum_def = peek_enum(shape).unwrap();
        assert_eq!(enum_def.repr, EnumRepr::U8);
        assert_eq!(enum_def.variants.len(), 3);
    }

    #[test]
    fn test_peek_enum_repr() {
        let shape = TestEnum::SHAPE;
        let repr = peek_enum_repr(shape).unwrap();
        assert_eq!(repr, EnumRepr::U8);
    }

    #[test]
    fn test_peek_enum_variants() {
        let shape = TestEnum::SHAPE;
        let variants = peek_enum_variants(shape).unwrap();
        assert_eq!(variants.len(), 3);
        assert_eq!(variants[0].name, "A");
        assert_eq!(variants[1].name, "B");
        assert_eq!(variants[2].name, "C");
    }
    */
}
