/// Log levels
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Global log level filter
static LOG_LEVEL: core::sync::atomic::AtomicU8 = core::sync::atomic::AtomicU8::new(Level::Info as u8);

/// Set the global log level
pub fn set_max_level(level: Level) {
    LOG_LEVEL.store(level as u8, core::sync::atomic::Ordering::Relaxed);
}

/// Check if a log level is enabled
pub fn log_enabled(level: Level) -> bool {
    level as u8 <= LOG_LEVEL.load(core::sync::atomic::Ordering::Relaxed)
}

/// Log a message
pub fn log(level: Level, args: core::fmt::Arguments) {
    if log_enabled(level) {
        // Simple implementation that just panics with the message
        // In a real implementation, this would output to a serial port or other device
        panic!("{}: {}", match level {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        }, args);
    }
}

/// Log macros
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if $crate::log::log_enabled($crate::log::Level::Error) {
            $crate::log::log($crate::log::Level::Error, core::format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        if $crate::log::log_enabled($crate::log::Level::Warn) {
            $crate::log::log($crate::log::Level::Warn, core::format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if $crate::log::log_enabled($crate::log::Level::Info) {
            $crate::log::log($crate::log::Level::Info, core::format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if $crate::log::log_enabled($crate::log::Level::Debug) {
            $crate::log::log($crate::log::Level::Debug, core::format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        if $crate::log::log_enabled($crate::log::Level::Trace) {
            $crate::log::log($crate::log::Level::Trace, core::format_args!($($arg)*));
        }
    };
}
