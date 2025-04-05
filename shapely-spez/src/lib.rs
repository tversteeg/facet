//! Tricks for specialization on Rust stable
//!
//! Completely missing docs, sorry, read this for background:
//! <https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html>

#![allow(missing_docs)]
pub use ::impls::impls;
use core::fmt::{self, Debug};
use shapely_types::ParseError;

use shapely_opaque::{Opaque, OpaqueUninit};

pub struct Spez<T>(pub T);

//////////////////////////////////////////////////////////////////////////////////////
// Debug 🐛🔍
//////////////////////////////////////////////////////////////////////////////////////

/// Specialization proxy for [`std::fmt::Debug`]
pub trait SpezDebugYes {
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
}
impl<T: Debug> SpezDebugYes for &Spez<T> {
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Debug::fmt(&self.0, f)
    }
}

/// Specialization proxy for [`std::fmt::Debug`]
pub trait SpezDebugNo {
    fn spez_debug(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
}
impl<T> SpezDebugNo for Spez<T> {
    fn spez_debug(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        unreachable!()
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// Display 📺🖥️
//////////////////////////////////////////////////////////////////////////////////////

/// Specialization proxy for [`std::fmt::Display`]
pub trait SpezDisplayYes {
    fn spez_display(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
}
impl<T: fmt::Display> SpezDisplayYes for &Spez<T> {
    fn spez_display(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&self.0, f)
    }
}

/// Specialization proxy for [`std::fmt::Display`]
pub trait SpezDisplayNo {
    fn spez_display(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
}
impl<T> SpezDisplayNo for Spez<T> {
    fn spez_display(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        unreachable!()
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// Default (in place, because we can't have sized) 🏠🔄
//////////////////////////////////////////////////////////////////////////////////////

/// Specialization proxy for [`std::default::Default`]
pub trait SpezDefaultInPlaceYes {
    fn spez_default_in_place<'mem>(&self, target: OpaqueUninit<'mem>) -> Opaque<'mem>;
}
impl<T: Default> SpezDefaultInPlaceYes for &Spez<T> {
    fn spez_default_in_place<'mem>(&self, target: OpaqueUninit<'mem>) -> Opaque<'mem> {
        unsafe { target.write(<T as Default>::default()) }
    }
}

/// Specialization proxy for [`std::default::Default`]
pub trait SpezDefaultInPlaceNo {
    fn spez_default_in_place<'mem>(&self, _target: OpaqueUninit<'mem>) -> Opaque<'mem>;
}
impl<T> SpezDefaultInPlaceNo for Spez<T> {
    fn spez_default_in_place<'mem>(&self, _target: OpaqueUninit<'mem>) -> Opaque<'mem> {
        unreachable!()
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// Clone into 🐑📥
//////////////////////////////////////////////////////////////////////////////////////

/// Specialization proxy for [`std::clone::Clone`]
pub trait SpezCloneIntoYes {
    fn spez_clone_into<'mem>(&self, target: OpaqueUninit<'mem>) -> Opaque<'mem>;
}
impl<T: Clone> SpezCloneIntoYes for &Spez<T> {
    fn spez_clone_into<'mem>(&self, target: OpaqueUninit<'mem>) -> Opaque<'mem> {
        unsafe { target.write(self.0.clone()) }
    }
}

/// Specialization proxy for [`std::clone::Clone`]
pub trait SpezCloneIntoNo {
    fn spez_clone_into<'mem>(&self, _target: OpaqueUninit<'mem>) -> Opaque<'mem>;
}
impl<T> SpezCloneIntoNo for Spez<T> {
    fn spez_clone_into<'mem>(&self, _target: OpaqueUninit<'mem>) -> Opaque<'mem> {
        unreachable!()
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// Parse 📝🔍
//////////////////////////////////////////////////////////////////////////////////////

/// Specialization proxy for [`std::str::FromStr`]
pub trait SpezParseYes {
    fn spez_parse(&self, s: &str, target: OpaqueUninit) -> Result<(), ParseError>;
}
impl<T: core::str::FromStr> SpezParseYes for &Spez<T> {
    fn spez_parse(&self, s: &str, target: OpaqueUninit) -> Result<(), ParseError> {
        match <T as core::str::FromStr>::from_str(s) {
            Ok(value) => {
                unsafe { target.write(value) };
                Ok(())
            }
            Err(_) => Err(ParseError::Generic(
                const { concat!("parse error for ", stringify!(T)) },
            )),
        }
    }
}

/// Specialization proxy for [`std::str::FromStr`]
pub trait SpezParseNo {
    fn spez_parse(&self, _s: &str, _target: OpaqueUninit) -> Result<(), ParseError>;
}
impl<T> SpezParseNo for Spez<T> {
    fn spez_parse(&self, _s: &str, _target: OpaqueUninit) -> Result<(), ParseError> {
        unreachable!()
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// PartialEq 🟰🤝
//////////////////////////////////////////////////////////////////////////////////////

/// Specialization proxy for [`std::cmp::PartialEq`]
pub trait SpezPartialEqYes {
    fn spez_eq(&self, other: &Self) -> bool;
}
impl<T: PartialEq> SpezPartialEqYes for &Spez<T> {
    fn spez_eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// Specialization proxy for [`std::cmp::PartialEq`]
pub trait SpezPartialEqNo {
    fn spez_eq(&self, _other: &Self) -> bool;
}
impl<T> SpezPartialEqNo for Spez<T> {
    fn spez_eq(&self, _other: &Self) -> bool {
        unreachable!()
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// PartialOrd 🔢↕️
//////////////////////////////////////////////////////////////////////////////////////

/// Specialization proxy for [`std::cmp::PartialOrd`]
pub trait SpezPartialOrdYes {
    fn spez_partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>;
}
impl<T: PartialOrd> SpezPartialOrdYes for &Spez<T> {
    fn spez_partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

/// Specialization proxy for [`std::cmp::PartialOrd`]
pub trait SpezPartialOrdNo {
    fn spez_partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering>;
}
impl<T> SpezPartialOrdNo for Spez<T> {
    fn spez_partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        unreachable!()
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// Ord 📊🔀
//////////////////////////////////////////////////////////////////////////////////////

/// Specialization proxy for [`std::cmp::Ord`]
pub trait SpezOrdYes {
    fn spez_cmp(&self, other: &Self) -> std::cmp::Ordering;
}
impl<T: Ord> SpezOrdYes for &Spez<T> {
    fn spez_cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

/// Specialization proxy for [`std::cmp::Ord`]
pub trait SpezOrdNo {
    fn spez_cmp(&self, _other: &Self) -> std::cmp::Ordering;
}
impl<T> SpezOrdNo for Spez<T> {
    fn spez_cmp(&self, _other: &Self) -> std::cmp::Ordering {
        unreachable!()
    }
}

//////////////////////////////////////////////////////////////////////////////////////
// Hash #️⃣🔐
//////////////////////////////////////////////////////////////////////////////////////

/// Specialization proxy for [`std::hash::Hash`]
pub trait SpezHashYes {
    fn spez_hash<H: std::hash::Hasher>(&self, state: &mut H);
}
impl<T: std::hash::Hash> SpezHashYes for &Spez<T> {
    fn spez_hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

/// Specialization proxy for [`std::hash::Hash`]
pub trait SpezHashNo {
    fn spez_hash<H: std::hash::Hasher>(&self, _state: &mut H);
}
impl<T> SpezHashNo for Spez<T> {
    fn spez_hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        unreachable!()
    }
}
