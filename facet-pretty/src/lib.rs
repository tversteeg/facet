#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod ansi;
mod color;
mod display;
mod printer;

pub use ansi::*;
pub use color::*;
pub use display::*;
pub use printer::*;

// Re-export facet_core for convenience
pub use facet_core;
