//! Tricks for specialization on Rust stable
//!
//! Completely missing docs, sorry, read this for background:
//! <https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html>

#![allow(missing_docs)]

use core::fmt::{self, Debug};

use crate::{Opaque, OpaqueUninit};

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
    fn spez_debug(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
}
impl<T> SpezDebugNo for Spez<T> {
    fn spez_debug(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
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
    fn spez_display(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
}
impl<T> SpezDisplayNo for Spez<T> {
    fn spez_display(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        unreachable!()
    }
}

////////////////////////////////////////////////////////////////////////////////////////
// Default (in place, because we can't have sized)
////////////////////////////////////////////////////////////////////////////////////////

pub trait SpezDefaultInPlaceYes {
    fn spez_default_in_place<'mem>(&self, target: OpaqueUninit<'mem>) -> Opaque<'mem>;
}
impl<T: Default> SpezDefaultInPlaceYes for &Spez<T> {
    fn spez_default_in_place<'mem>(&self, target: OpaqueUninit<'mem>) -> Opaque<'mem> {
        unsafe { target.write(<T as Default>::default()) }
    }
}

pub trait SpezDefaultInPlaceNo {
    fn spez_default_in_place<'mem>(&self, _target: OpaqueUninit<'mem>) -> Opaque<'mem>;
}
impl<T> SpezDefaultInPlaceNo for Spez<T> {
    fn spez_default_in_place<'mem>(&self, _target: OpaqueUninit<'mem>) -> Opaque<'mem> {
        unreachable!()
    }
}

////////////////////////////////////////////////////////////////////////////////////////
// Parse
////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct ParseError;

pub trait SpezParseYes {
    fn spez_parse(&self, s: &str, target: OpaqueUninit) -> Result<(), ParseError>;
}
impl<T: core::str::FromStr> SpezParseYes for Spez<T> {
    fn spez_parse(&self, s: &str, target: OpaqueUninit) -> Result<(), ParseError> {
        match <T as core::str::FromStr>::from_str(s) {
            Ok(value) => {
                unsafe { target.write(value) };
                Ok(())
            }
            Err(_) => Err(ParseError),
        }
    }
}

pub trait SpezParseNo {
    fn spez_parse(&self, _s: &str, _target: OpaqueUninit) -> Result<(), ParseError>;
}
impl<T> SpezParseNo for Spez<T> {
    fn spez_parse(&self, _s: &str, _target: OpaqueUninit) -> Result<(), ParseError> {
        unreachable!()
    }
}

// For Eq
pub trait SpezEqYes {
    fn spez_eq(&self, other: &Self) -> bool;
}
impl<T: PartialEq> SpezEqYes for &Spez<T> {
    fn spez_eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub trait SpezEqNo {
    fn spez_eq(&self, _other: &Self) -> bool;
}
impl<T> SpezEqNo for Spez<T> {
    fn spez_eq(&self, _other: &Self) -> bool {
        unreachable!()
    }
}

// For Cmp
pub trait SpezCmpYes {
    fn spez_cmp(&self, other: &Self) -> std::cmp::Ordering;
}
impl<T: Ord> SpezCmpYes for &Spez<T> {
    fn spez_cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

pub trait SpezCmpNo {
    fn spez_cmp(&self, _other: &Self) -> std::cmp::Ordering;
}
impl<T> SpezCmpNo for Spez<T> {
    fn spez_cmp(&self, _other: &Self) -> std::cmp::Ordering {
        unreachable!()
    }
}

// For Hash
pub trait SpezHashYes {
    fn spez_hash<H: std::hash::Hasher>(&self, state: &mut H);
}
impl<T: std::hash::Hash> SpezHashYes for &Spez<T> {
    fn spez_hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

pub trait SpezHashNo {
    fn spez_hash<H: std::hash::Hasher>(&self, _state: &mut H);
}
impl<T> SpezHashNo for Spez<T> {
    fn spez_hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        unreachable!()
    }
}
