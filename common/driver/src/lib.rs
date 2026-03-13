//! FeatherCore Driver Library
//! FeatherCore 驱动库
//! 
//! This library provides device driver interfaces for FeatherCore.
//! 这个库为 FeatherCore 提供设备驱动接口。

#![no_std]

// Driver framework
// 驱动框架
pub mod led;
pub mod gpio;
pub mod i2c;
pub mod serial;
pub mod spi;
pub mod i2s;
pub mod timer;
