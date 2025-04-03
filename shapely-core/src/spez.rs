//! Tricks for specialization on Rust stable

use core::fmt;

/// proxies, using the <https://docs.rs/spez> trick, kind of:
pub struct Spez<T>(pub T);

/// Trait for types that can be debugged via the `Debug` trait
pub trait ViaDebug {
    /// Attempts to format self using the `Debug` trait
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
}

/// Trait for types that cannot be debugged
pub trait ViaNone {
    /// Always returns None, indicating the type cannot be debugged
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
}

impl<T: fmt::Debug> ViaDebug for &Spez<T> {
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T> ViaNone for Spez<T> {
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        unreachable!()
    }
}
