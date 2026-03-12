//! Timer/Counter Driver Common Layer (Core)
//! 定时器/计数器驱动通用核心层
//!
//! This module provides the common timer/counter driver interface that is independent
//! of specific hardware platforms. It defines the traits and structures
//! that all platform-specific timer/counter drivers must implement.
//! 该模块提供与硬件平台无关的通用定时器/计数器驱动接口。它定义了所有平台特定
//! 定时器/计数器驱动必须实现的特征和结构。
//!
//! # Architecture / 架构
//! - **Common Core Layer**: Defines abstract interfaces (traits) for timer operations
//! - **Platform Adapter Layer**: Implements the interfaces for specific hardware (e.g., STM32)
//!
//! # Usage Example / 使用示例
//! ```ignore
//! use driver::timer::{TimerDriver, TimerConfig, TimerChannel, Result};
//!
//! // Initialize timer with platform-specific driver
//! let mut timer = TimerDriver::new(stm32_timer::CONFIG)?;
//! timer.init(&TimerConfig::default())?;
//!
//! // Start timer
//! timer.start()?;
//!
//! // Set alarm for channel 0
//! timer.set_alarm(0, 1000, callback)?;
//!
//! // Get current counter value
//! let value = timer.get_value()?;
//! ```

pub mod traits;
pub mod config;
pub mod error;

pub use traits::TimerDriver;
pub use config::{TimerConfig, TimerChannel, TimerMode, TimerPrescaler};
pub use error::TimerError;

pub mod platform {
    //! Platform-specific timer driver implementations
    //! 平台特定的定时器驱动实现

    #[cfg(feature = "stm32f4")]
    pub mod stm32f4 {
        //! STM32F4 Series Timer Driver
        //! STM32F4 系列定时器驱动

        pub mod timer;
    }
}
