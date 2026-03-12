//! I2S Driver Common Layer (Core)
//! I2S 驱动通用核心层
//!
//! This module provides the common I2S driver interface that is independent
//! of specific hardware platforms. It defines the traits and structures
//! that all platform-specific I2S drivers must implement.
//! 该模块提供与硬件平台无关的通用 I2S 驱动接口。它定义了所有平台特定
//! I2S 驱动必须实现的特征和结构。
//!
//! # Architecture / 架构
//! - **Common Core Layer**: Defines abstract interfaces (traits) for I2S operations
//! - **Platform Adapter Layer**: Implements the interfaces for specific hardware (e.g., STM32)
//!
//! # Usage Example / 使用示例
//! ```ignore
//! use driver::i2s::{I2sDriver, I2sConfig, I2sFormat, I2sMode, Result};
//!
//! // Initialize I2S with platform-specific driver
//! let mut i2s = I2sDriver::new(stm32_i2s::CONFIG)?;
//! i2s.init(&I2sConfig::default())?;
//!
//! // Configure for TX mode
//! i2s.configure(I2sDir::Tx, &config)?;
//!
//! // Write audio data
//! i2s.write(buffer)?;
//! ```

pub mod traits;
pub mod config;
pub mod error;

pub use traits::I2sDriver;
pub use config::{I2sConfig, I2sFormat, I2sMode, I2sDir, I2sWordSize, I2sClockPolarity, I2sFrameSync};
pub use error::I2sError;

pub mod platform {
    //! Platform-specific I2S driver implementations
    //! 平台特定的 I2S 驱动实现

    #[cfg(feature = "stm32f4")]
    pub mod stm32f4 {
        //! STM32F4 Series I2S Driver
        //! STM32F4 系列 I2S 驱动

        pub mod i2s;
    }
}
