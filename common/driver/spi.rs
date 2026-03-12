//! SPI Driver Common Layer (Core)
//! SPI 驱动通用核心层
//!
//! This module provides the common SPI driver interface that is independent
//! of specific hardware platforms. It defines the traits and structures
//! that all platform-specific SPI drivers must implement.
//! 该模块提供与硬件平台无关的通用 SPI 驱动接口。它定义了所有平台特定
//! SPI 驱动必须实现的特征和结构。
//!
//! # Architecture / 架构
//! - **Common Core Layer**: Defines abstract interfaces (traits) for SPI operations
//! - **Platform Adapter Layer**: Implements the interfaces for specific hardware (e.g., STM32)
//!
//! # Usage Example / 使用示例
//! ```ignore
//! use driver::spi::{SpiDriver, SpiConfig, SpiMode, SpiFormat, Result};
//!
//! // Initialize SPI with platform-specific driver
//! let mut spi = SpiDriver::new(stm32_spi::CONFIG)?;
//! spi.init(&SpiConfig::default())?;
//!
//! // Configure for master mode
//! spi.configure(&SpiConfig::master())?;
//!
//! // Transfer data
//! let tx_buffer = [0x01, 0x02, 0x03];
//! let mut rx_buffer = [0u8; 3];
//! spi.transfer(&tx_buffer, &mut rx_buffer)?;
//! ```

pub mod traits;
pub mod config;
pub mod error;

pub use traits::SpiDriver;
pub use config::{SpiConfig, SpiMode, SpiFormat, SpiOperation, SpiBitOrder, SpiDataLines};
pub use error::SpiError;

pub mod platform {
    //! Platform-specific SPI driver implementations
    //! 平台特定的 SPI 驱动实现

    #[cfg(feature = "stm32f4")]
    pub mod stm32f4 {
        //! STM32F4 Series SPI Driver
        //! STM32F4 系列 SPI 驱动

        pub mod spi;
    }
}
