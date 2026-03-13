//! FeatherCore Driver Library
//! FeatherCore 驱动库

#![no_std]

pub mod gpio;
pub mod i2c;
pub mod serial;
pub mod spi;
pub mod i2s;
pub mod timer;

pub mod led {
    //! LED Driver / LED 驱动

    /// LED error types / LED 错误类型
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum LedError {
        InvalidPin,
        NotInitialized,
    }

    /// LED driver trait / LED 驱动特征
    pub trait LedDriver {
        fn init(&mut self) -> Result<(), LedError>;
        fn on(&mut self) -> Result<(), LedError>;
        fn off(&mut self) -> Result<(), LedError>;
    }
}
