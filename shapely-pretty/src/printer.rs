//! Pretty printer implementation for Shapely types

use std::{
    collections::HashSet,
    fmt::{self, Write},
    hash::{DefaultHasher, Hash, Hasher},
};

use shapely_core::{Innards, Scalar, Shape, ShapeDesc, Shapely};

use crate::{ansi, color::ColorGenerator};

/// A formatter for pretty-printing Shapely types
pub struct PrettyPrinter {
    indent_size: usize,
    max_depth: Option<usize>,
    color_generator: ColorGenerator,
    use_colors: bool,
}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self {
            indent_size: 2,
            max_depth: None,
            color_generator: ColorGenerator::default(),
            use_colors: true,
        }
    }
}

impl PrettyPrinter {
    /// Create a new PrettyPrinter with default settings
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the indentation size
    pub fn with_indent_size(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }
    
    /// Set the maximum depth for recursive printing
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }
    
    /// Set the color generator
    pub fn with_color_generator(mut self, generator: ColorGenerator) -> Self {
        self.color_generator = generator;
        self
    }
    
    /// Enable or disable colors
    pub fn with_colors(mut self, use_colors: bool) -> Self {
        self.use_colors = use_colors;
        self
    }
    
    /// Pretty-print a value that implements Shapely
    pub fn print<T: Shapely>(&self, value: &T) {
        let shape_desc = T::shape_desc();
        let ptr = value as *const T as *mut u8;
        
        let mut output = String::new();
        self.format_value(ptr, shape_desc, &mut output, 0, &mut HashSet::new())
            .expect("Formatting failed");
        
        print!("{}", output);
    }
    
    /// Format a value to a string
    pub fn format<T: Shapely>(&self, value: &T) -> String {
        let shape_desc = T::shape_desc();
        let ptr = value as *const T as *mut u8;
        
        let mut output = String::new();
        self.format_value(ptr, shape_desc, &mut output, 0, &mut HashSet::new())
            .expect("Formatting failed");
        
        output
    }
    
    /// Format a value to a formatter
    pub fn format_to<T: Shapely>(&self, value: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shape_desc = T::shape_desc();
        let ptr = value as *const T as *mut u8;
        
        self.format_value(ptr, shape_desc, f, 0, &mut HashSet::new())
    }
    
    /// Internal method to format a value at a specific memory address
    pub(crate) fn format_value(
        &self,
        ptr: *mut u8,
        shape_desc: ShapeDesc,
        f: &mut impl Write,
        depth: usize,
        visited: &mut HashSet<*mut u8>,
    ) -> fmt::Result {
        // Check if we've reached the maximum depth
        if let Some(max_depth) = self.max_depth {
            if depth > max_depth {
                return write!(f, "{}...", self.style_punctuation("["));
            }
        }
        
        // Get the shape
        let shape = shape_desc.get();
        
        // Generate a color for this shape
        let mut hasher = DefaultHasher::new();
        shape.typeid.hash(&mut hasher);
        let hash = hasher.finish();
        let (r, g, b) = self.color_generator.generate_color(hash);
        
        // Format based on the shape's innards
        match &shape.innards {
            Innards::Scalar(scalar) => {
                self.format_scalar(ptr, *scalar, f, r, g, b)
            }
            Innards::Struct { fields } => {
                self.format_struct(ptr, shape, fields, f, depth, visited, r, g, b)
            }
            Innards::HashMap { value_shape } => {
                self.format_hashmap(ptr, shape, *value_shape, f, depth, visited, r, g, b)
            }
            Innards::Array(elem_shape) => {
                self.format_array(ptr, shape, *elem_shape, f, depth, visited, r, g, b)
            }
            Innards::Transparent(inner_shape) => {
                self.format_transparent(ptr, shape, *inner_shape, f, depth, visited, r, g, b)
            }
        }
    }
    
    /// Format a scalar value
    fn format_scalar(
        &self,
        _ptr: *mut u8,
        scalar: Scalar,
        f: &mut impl Write,
        r: u8,
        g: u8,
        b: u8,
    ) -> fmt::Result {
        // This is a simplified implementation - in a real implementation,
        // we would need to read the actual value from the pointer
        // For now, we'll just print the scalar type with a placeholder
        
        let scalar_color = if self.use_colors {
            ansi::rgb(r, g, b)
        } else {
            String::new()
        };
        
        let reset = if self.use_colors { ansi::RESET } else { "" };
        
        match scalar {
            Scalar::String => write!(f, "{}\"<string>\"{}",  scalar_color, reset),
            Scalar::Bytes => write!(f, "{}b\"<bytes>\"{}",  scalar_color, reset),
            Scalar::I8 => write!(f, "{}<i8>{}",  scalar_color, reset),
            Scalar::I16 => write!(f, "{}<i16>{}",  scalar_color, reset),
            Scalar::I32 => write!(f, "{}<i32>{}",  scalar_color, reset),
            Scalar::I64 => write!(f, "{}<i64>{}",  scalar_color, reset),
            Scalar::I128 => write!(f, "{}<i128>{}",  scalar_color, reset),
            Scalar::U8 => write!(f, "{}<u8>{}",  scalar_color, reset),
            Scalar::U16 => write!(f, "{}<u16>{}",  scalar_color, reset),
            Scalar::U32 => write!(f, "{}<u32>{}",  scalar_color, reset),
            Scalar::U64 => write!(f, "{}<u64>{}",  scalar_color, reset),
            Scalar::U128 => write!(f, "{}<u128>{}",  scalar_color, reset),
            Scalar::F32 => write!(f, "{}<f32>{}",  scalar_color, reset),
            Scalar::F64 => write!(f, "{}<f64>{}",  scalar_color, reset),
            Scalar::Boolean => write!(f, "{}<bool>{}",  scalar_color, reset),
            Scalar::Nothing => write!(f, "{}(){}",  scalar_color, reset),
            // Handle any future scalar types
            _ => write!(f, "{}<unknown scalar>{}",  scalar_color, reset),
        }
    }
    
    /// Format a struct
    fn format_struct(
        &self,
        ptr: *mut u8,
        shape: Shape,
        fields: &'static [shapely_core::Field],
        f: &mut impl Write,
        depth: usize,
        visited: &mut HashSet<*mut u8>,
        _r: u8,
        _g: u8,
        _b: u8,
    ) -> fmt::Result {
        // Check for cycles
        if !visited.insert(ptr) {
            return write!(f, "{}{}{}{}{}",
                self.style_type_name(&shape.to_string()),
                self.style_punctuation(" { "),
                self.style_comment("/* cycle detected */"),
                self.style_punctuation(" }"),
                if self.use_colors { ansi::RESET } else { "" }
            );
        }
        
        // Print the struct name
        write!(f, "{}", self.style_type_name(&shape.to_string()))?;
        write!(f, "{}", self.style_punctuation(" {"))?;
        
        if fields.is_empty() {
            write!(f, "{}", self.style_punctuation(" }"))?;
            visited.remove(&ptr);
            return Ok(());
        }
        
        writeln!(f)?;
        
        // Print each field
        for field in fields {
            // Indent
            write!(f, "{:width$}", "", width = (depth + 1) * self.indent_size)?;
            
            // Field name
            write!(f, "{}: ", self.style_field_name(field.name))?;
            
            // Field value - compute the field address
            let field_ptr = unsafe { ptr.add(field.offset) };
            self.format_value(field_ptr, field.shape, f, depth + 1, visited)?;
            
            writeln!(f, "{}", self.style_punctuation(","))?;
        }
        
        // Closing brace with proper indentation
        write!(f, "{:width$}{}", "", self.style_punctuation("}"), width = depth * self.indent_size)?;
        
        // Remove from visited set when we're done with this struct
        visited.remove(&ptr);
        
        Ok(())
    }
    
    /// Format a HashMap
    fn format_hashmap(
        &self,
        _ptr: *mut u8,
        shape: Shape,
        _value_shape: ShapeDesc,
        f: &mut impl Write,
        depth: usize,
        _visited: &mut HashSet<*mut u8>,
        _r: u8,
        _g: u8,
        _b: u8,
    ) -> fmt::Result {
        // In a real implementation, we would need to iterate over the HashMap
        // For now, we'll just print a placeholder
        
        write!(f, "{}", self.style_type_name(&shape.to_string()))?;
        write!(f, "{}", self.style_punctuation(" {"))?;
        writeln!(f)?;
        
        // Indent
        write!(f, "{:width$}", "", width = (depth + 1) * self.indent_size)?;
        write!(f, "{}", self.style_comment("/* HashMap contents */"))?;
        writeln!(f)?;
        
        // Closing brace with proper indentation
        write!(f, "{:width$}{}", "", self.style_punctuation("}"), width = depth * self.indent_size)
    }
    
    /// Format an array
    fn format_array(
        &self,
        _ptr: *mut u8,
        shape: Shape,
        _elem_shape: ShapeDesc,
        f: &mut impl Write,
        depth: usize,
        _visited: &mut HashSet<*mut u8>,
        _r: u8,
        _g: u8,
        _b: u8,
    ) -> fmt::Result {
        // In a real implementation, we would need to iterate over the array
        // For now, we'll just print a placeholder
        
        write!(f, "{}", self.style_type_name(&shape.to_string()))?;
        write!(f, "{}", self.style_punctuation(" ["))?;
        writeln!(f)?;
        
        // Indent
        write!(f, "{:width$}", "", width = (depth + 1) * self.indent_size)?;
        write!(f, "{}", self.style_comment("/* Array contents */"))?;
        writeln!(f)?;
        
        // Closing bracket with proper indentation
        write!(f, "{:width$}{}", "", self.style_punctuation("]"), width = depth * self.indent_size)
    }
    
    /// Format a transparent wrapper
    fn format_transparent(
        &self,
        ptr: *mut u8,
        shape: Shape,
        inner_shape: ShapeDesc,
        f: &mut impl Write,
        depth: usize,
        visited: &mut HashSet<*mut u8>,
        _r: u8,
        _g: u8,
        _b: u8,
    ) -> fmt::Result {
        // Print the wrapper type name
        write!(f, "{}", self.style_type_name(&shape.to_string()))?;
        write!(f, "{}", self.style_punctuation("("))?;
        
        // Format the inner value
        self.format_value(ptr, inner_shape, f, depth, visited)?;
        
        // Closing parenthesis
        write!(f, "{}", self.style_punctuation(")"))
    }
    
    /// Style a type name
    fn style_type_name(&self, name: &str) -> String {
        if self.use_colors {
            format!("{}{}{}", ansi::BOLD, name, ansi::RESET)
        } else {
            name.to_string()
        }
    }
    
    /// Style a field name
    fn style_field_name(&self, name: &str) -> String {
        if self.use_colors {
            format!("{}{}{}", ansi::rgb(114, 160, 193), name, ansi::RESET)
        } else {
            name.to_string()
        }
    }
    
    /// Style punctuation
    fn style_punctuation(&self, text: &str) -> String {
        if self.use_colors {
            format!("{}{}{}", ansi::DIM, text, ansi::RESET)
        } else {
            text.to_string()
        }
    }
    
    /// Style a comment
    fn style_comment(&self, text: &str) -> String {
        if self.use_colors {
            format!("{}{}{}", ansi::DIM, text, ansi::RESET)
        } else {
            text.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Basic tests for the PrettyPrinter
    #[test]
    fn test_pretty_printer_default() {
        let printer = PrettyPrinter::default();
        assert_eq!(printer.indent_size, 2);
        assert_eq!(printer.max_depth, None);
        assert_eq!(printer.use_colors, true);
    }
    
    #[test]
    fn test_pretty_printer_with_methods() {
        let printer = PrettyPrinter::new()
            .with_indent_size(4)
            .with_max_depth(3)
            .with_colors(false);
            
        assert_eq!(printer.indent_size, 4);
        assert_eq!(printer.max_depth, Some(3));
        assert_eq!(printer.use_colors, false);
    }
    
    #[test]
    fn test_style_methods() {
        let printer_with_colors = PrettyPrinter::new().with_colors(true);
        let printer_without_colors = PrettyPrinter::new().with_colors(false);
        
        // With colors
        assert_eq!(
            printer_with_colors.style_type_name("Test"),
            format!("{}Test{}", ansi::BOLD, ansi::RESET)
        );
        
        // Without colors
        assert_eq!(
            printer_without_colors.style_type_name("Test"),
            "Test"
        );
    }
}