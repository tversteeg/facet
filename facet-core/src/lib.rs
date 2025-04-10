#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub use typeid::ConstTypeId;

// Opaque pointer utilities
mod opaque;
pub use opaque::*;

// Specialization utilities
pub mod spez;

// Core trait definitions
mod _trait;
pub use _trait::*;

// Type definitions
mod types;
#[allow(unused_imports)] // wtf clippy? we're re-exporting?
pub use types::*;
