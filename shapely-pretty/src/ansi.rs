//! ANSI color codes and formatting utilities

/// ANSI escape code prefix
pub const ESC: &str = "\x1b[";

/// ANSI reset code
pub const RESET: &str = "\x1b[0m";

/// ANSI bold formatting
pub const BOLD: &str = "\x1b[1m";

/// ANSI dim formatting
pub const DIM: &str = "\x1b[2m";

/// ANSI italic formatting
pub const ITALIC: &str = "\x1b[3m";

/// ANSI underline formatting
pub const UNDERLINE: &str = "\x1b[4m";

/// Create an RGB foreground color code
pub fn rgb(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

/// Create an RGB background color code
pub fn rgb_bg(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{};{};{}m", r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_format() {
        assert_eq!(rgb(255, 0, 0), "\x1b[38;2;255;0;0m");
        assert_eq!(rgb(0, 255, 0), "\x1b[38;2;0;255;0m");
        assert_eq!(rgb(0, 0, 255), "\x1b[38;2;0;0;255m");
    }

    #[test]
    fn test_rgb_bg_format() {
        assert_eq!(rgb_bg(255, 0, 0), "\x1b[48;2;255;0;0m");
        assert_eq!(rgb_bg(0, 255, 0), "\x1b[48;2;0;255;0m");
        assert_eq!(rgb_bg(0, 0, 255), "\x1b[48;2;0;0;255m");
    }
}