use crate::ShapeDesc;

/// Describes a field in a struct or tuple
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Field {
    /// key for the struct field (for tuples and tuple-structs, this is the 0-based index)
    pub name: &'static str,

    /// schema of the inner type
    pub shape: ShapeDesc,

    /// offset of the field in the struct (obtained through `std::mem::offset_of`)
    pub offset: usize,

    /// flags for the field (e.g. sensitive, etc.)
    pub flags: FieldFlags,
}

/// Flags that can be applied to fields to modify their behavior
///
/// # Examples
///
/// ```rust
/// use shapely_core::FieldFlags;
///
/// // Create flags with the sensitive bit set
/// let flags = FieldFlags::SENSITIVE;
/// assert!(flags.contains(FieldFlags::SENSITIVE));
///
/// // Combine multiple flags using bitwise OR
/// let flags = FieldFlags::SENSITIVE | FieldFlags::EMPTY;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldFlags(u64);

impl FieldFlags {
    /// An empty set of flags
    pub const EMPTY: Self = Self(0);

    /// Flag indicating this field contains sensitive data that should not be displayed
    pub const SENSITIVE: Self = Self(1 << 0);

    /// Returns true if the given flag is set
    #[inline]
    pub fn contains(&self, flag: FieldFlags) -> bool {
        self.0 & flag.0 != 0
    }

    /// Sets the given flag and returns self for chaining
    #[inline]
    pub fn set_flag(&mut self, flag: FieldFlags) -> &mut Self {
        self.0 |= flag.0;
        self
    }

    /// Unsets the given flag and returns self for chaining
    #[inline]
    pub fn unset_flag(&mut self, flag: FieldFlags) -> &mut Self {
        self.0 &= !flag.0;
        self
    }

    /// Creates a new FieldFlags with the given flag set
    #[inline]
    pub const fn with_flag(flag: FieldFlags) -> Self {
        Self(flag.0)
    }
}

impl std::ops::BitOr for FieldFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for FieldFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Default for FieldFlags {
    #[inline(always)]
    fn default() -> Self {
        Self::EMPTY
    }
}

impl std::fmt::Display for FieldFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            return write!(f, "none");
        }

        // Define a vector of flag entries: (flag bit, name)
        let flags = [
            (Self::SENSITIVE.0, "sensitive"),
            // Future flags can be easily added here:
            // (Self::SOME_FLAG.0, "some_flag"),
            // (Self::ANOTHER_FLAG.0, "another_flag"),
        ];

        // Write all active flags with proper separators
        let mut is_first = true;
        for (bit, name) in flags {
            if self.0 & bit != 0 {
                if !is_first {
                    write!(f, ", ")?;
                }
                is_first = false;
                write!(f, "{}", name)?;
            }
        }

        Ok(())
    }
}
