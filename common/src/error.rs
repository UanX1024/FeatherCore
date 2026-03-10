//! Error handling utilities

use crate::Error;

/// Extension trait for Result types
pub trait ResultExt<T> {
    /// Map an error to FeatherCore Error
    fn feather_err(self) -> Result<T>;
}

impl<T, E> ResultExt<T> for core::result::Result<T, E> {
    fn feather_err(self) -> Result<T> {
        self.map_err(|_| Error::Other)
    }
}

/// Check if a condition is true, otherwise return an error
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}

/// Return early with an error if a condition is not met
#[macro_export]
macro_rules! require {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}