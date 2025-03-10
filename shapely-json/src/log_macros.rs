#[cfg(feature = "log")]
pub use log::{error, trace, warn};

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {};
}
