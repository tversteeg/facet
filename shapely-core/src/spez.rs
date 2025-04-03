//! Tricks for specialization on Rust stable
//!
//! Completely missing docs, sorry, read this for background:
//! <https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html>

#![allow(missing_docs)]

use core::fmt::{self, Debug};

use crate::OpaqueUninit;

pub struct Spez<T>(pub T);

////////////////////////////////////////////////////////////////////////////////////////
// Debug
////////////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////////////
// Display
////////////////////////////////////////////////////////////////////////////////////////

pub trait SpezDisplayYes {
    fn spez_display(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
}
impl<T: fmt::Display> SpezDisplayYes for &Spez<T> {
    fn spez_display(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&self.0, f)
    }
}

pub trait SpezDisplayNo {
    fn spez_display(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
}
impl<T> SpezDisplayNo for Spez<T> {
    fn spez_display(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        unreachable!()
    }
}

////////////////////////////////////////////////////////////////////////////////////////
// Default (in place, because we can't have sized)
////////////////////////////////////////////////////////////////////////////////////////

pub trait SpezDefaultInPlaceYes {
    fn spez_default_in_place(&self, target: OpaqueUninit);
}
impl<T: Default> SpezDefaultInPlaceYes for Spez<T> {
    fn spez_default_in_place(&self, target: OpaqueUninit) {
        unsafe { target.write(<T as Default>::default()) };
    }
}

pub trait SpezDefaultInPlaceNo {
    fn spez_default_in_place(&self, target: OpaqueUninit);
}
impl<T> SpezDefaultInPlaceNo for Spez<T> {
    fn spez_default_in_place(&self, target: OpaqueUninit) {
        unreachable!()
    }
}

////////////////////////////////////////////////////////////////////////////////////////
// Parse
////////////////////////////////////////////////////////////////////////////////////////

pub trait SpezParseYes {
    fn spez_parse(&self, s: &str, target: OpaqueUninit) -> Result<(), ()>;
}
impl<T: core::str::FromStr> SpezParseYes for Spez<T> {
    fn spez_parse(&self, s: &str, target: OpaqueUninit) -> Result<(), ()> {
        match <T as core::str::FromStr>::from_str(s) {
            Ok(value) => {
                unsafe { target.write(value) };
                Ok(())
            }
            Err(_) => Err(()),
        }
    }
}

pub trait SpezParseNo {
    fn spez_parse(&self, s: &str, target: OpaqueUninit) -> Result<(), ()>;
}
impl<T> SpezParseNo for Spez<T> {
    fn spez_parse(&self, _s: &str, _target: OpaqueUninit) -> Result<(), ()> {
        unreachable!()
    }
}
