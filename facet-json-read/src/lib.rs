#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod parser;

mod deserialize;
pub use deserialize::*;
