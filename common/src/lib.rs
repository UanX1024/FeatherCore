//! FeatherCore Common Library
//!
//! This library provides common functionality shared between bootloader and kernel.
//! All code in this library must be `no_std` and should not depend on any external
//! libraries except architecture-specific code.

#![no_std]
#![cfg_attr(feature = "async", feature(async_fn_in_trait))]
#![cfg_attr(feature = "async", allow(incomplete_features))]
#![deny(missing_docs)]
#![deny(unsafe_code)]

// Re-export architecture modules if features are enabled
#[cfg(feature = "arm")]
pub use feathercore_arch_arm as arch;

#[cfg(feature = "riscv")]
pub use feathercore_arch_riscv as arch;

// Core modules
pub mod error;
pub mod util;

// Optional modules (enabled by features)
#[cfg(feature = "async")]
pub mod async_rt;

#[cfg(feature = "mm")]
pub mod mm;

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "driver")]
pub mod driver;

#[cfg(feature = "fs")]
pub mod fs;

/// Common error type for FeatherCore
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Invalid argument
    InvalidArgument,
    /// Operation not supported
    NotSupported,
    /// Resource busy
    Busy,
    /// Timeout
    Timeout,
    /// Out of memory
    OutOfMemory,
    /// Device error
    DeviceError,
    /// Filesystem error
    FilesystemError,
    /// Other error
    Other,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidArgument => write!(f, "Invalid argument"),
            Error::NotSupported => write!(f, "Operation not supported"),
            Error::Busy => write!(f, "Resource busy"),
            Error::Timeout => write!(f, "Operation timeout"),
            Error::OutOfMemory => write!(f, "Out of memory"),
            Error::DeviceError => write!(f, "Device error"),
            Error::FilesystemError => write!(f, "Filesystem error"),
            Error::Other => write!(f, "Other error"),
        }
    }
}

/// Result type for FeatherCore operations
pub type Result<T> = core::result::Result<T, Error>;