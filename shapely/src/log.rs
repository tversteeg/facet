#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warn => write!(f, "warn"),
            Self::Info => write!(f, "info"),
            Self::Debug => write!(f, "debug"),
            Self::Trace => write!(f, "trace"),
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::Level::Error, &format!($($arg)*))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::Level::Warn, &format!($($arg)*))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::Level::Info, &format!($($arg)*))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::Level::Debug, &format!($($arg)*))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::Level::Trace, &format!($($arg)*))
    };
}

#[doc(hidden)]
pub fn log(level: Level, message: &str) {
    let level_color = match level {
        Level::Error => "\x1b[31m", // Red
        Level::Warn => "\x1b[33m",  // Yellow
        Level::Info => "\x1b[32m",  // Green
        Level::Debug => "\x1b[36m", // Cyan
        Level::Trace => "\x1b[35m", // Magenta
    };
    let args_color = "\x1b[37m"; // White for args
    eprintln!(
        "{}{}:\x1b[0m {}{}\x1b[0m",
        level_color, level, args_color, message
    );
}
