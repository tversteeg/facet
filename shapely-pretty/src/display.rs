//! Display trait implementations for pretty-printing Shapely types

use std::fmt::{self, Display, Formatter};

use crate::printer::PrettyPrinter;

/// Display wrapper for any type that implements Shapely
pub struct PrettyDisplay<'a, T: facet_core::Shapely> {
    pub(crate) value: &'a T,
    pub(crate) printer: PrettyPrinter,
}

impl<T: facet_core::Shapely> Display for PrettyDisplay<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.printer.format_to(self.value, f)
    }
}

/// Extension trait for Shapely types to easily pretty-print them
pub trait ShapelyPretty: facet_core::Shapely {
    /// Get a displayable wrapper that pretty-prints this value
    fn pretty(&self) -> PrettyDisplay<'_, Self>;

    /// Get a displayable wrapper with custom printer settings
    fn pretty_with(&self, printer: PrettyPrinter) -> PrettyDisplay<'_, Self>;
}

impl<T: facet_core::Shapely> ShapelyPretty for T {
    fn pretty(&self) -> PrettyDisplay<'_, Self> {
        PrettyDisplay {
            value: self,
            printer: PrettyPrinter::default(),
        }
    }

    fn pretty_with(&self, printer: PrettyPrinter) -> PrettyDisplay<'_, Self> {
        PrettyDisplay {
            value: self,
            printer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use facet::Shapely;
    use std::fmt::Write;

    // Use the derive macro from facet
    #[derive(Shapely)]
    struct TestStruct {
        field: u32,
    }

    #[test]
    fn test_pretty_display() {
        let test = TestStruct { field: 42 };
        let display = test.pretty();

        let mut output = String::new();
        write!(output, "{}", display).unwrap();

        // Just check that it contains the field name and doesn't panic
        assert!(output.contains("field"));
    }

    #[test]
    fn test_pretty_with_custom_printer() {
        let test = TestStruct { field: 42 };
        let printer = PrettyPrinter::new().with_colors(false);
        let display = test.pretty_with(printer);

        let mut output = String::new();
        write!(output, "{}", display).unwrap();

        // Just check that it contains the field name and doesn't panic
        assert!(output.contains("field"));
    }
}
