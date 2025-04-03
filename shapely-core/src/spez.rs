//! Tricks for specialization on Rust stable
//!
//! Completely missing docs, sorry, read this for background:
//! <https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html>

#![allow(missing_docs)]

use core::fmt::{self, Debug};

pub struct Spez<T>(pub T);

pub trait SpezDebugYes {
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
}
impl<T: Debug> SpezDebugYes for &Spez<T> {
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Debug::fmt(&self.0, f)
    }
}

pub trait SpezDebugNo {
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
}
impl<T> SpezDebugNo for Spez<T> {
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        unreachable!()
    }
}
