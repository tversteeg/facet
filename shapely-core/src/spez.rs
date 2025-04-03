use core::fmt;

/// proxies, using the <https://docs.rs/spez> trick, kind of:
pub struct Spez<T>(pub T);

pub trait ViaDebug {
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Option<Result<(), fmt::Error>>;
}

pub trait ViaNone {
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Option<Result<(), fmt::Error>>;
}

impl<T: fmt::Debug> ViaDebug for &Spez<T> {
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Option<Result<(), fmt::Error>> {
        eprintln!("yes Debug");
        Some(fmt::Debug::fmt(&self.0, f))
    }
}

impl<T> ViaNone for Spez<T> {
    fn spez_debug(&self, f: &mut fmt::Formatter<'_>) -> Option<Result<(), fmt::Error>> {
        eprintln!("no Debug");
        None
    }
}
