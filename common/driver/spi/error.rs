//! SPI Driver Error Types
//! SPI 驱动错误类型
//!
//! This module defines the error types used by the SPI driver.
//! 该模块定义了 SPI 驱动使用的错误类型。

/// SPI driver error types
/// SPI 驱动错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiError {
    /// Invalid configuration / 无效配置
    InvalidConfig,
    /// Device not initialized / 设备未初始化
    NotInitialized,
    /// Device already initialized / 设备已初始化
    AlreadyInitialized,
    /// Buffer too small / 缓冲区太小
    BufferTooSmall,
    /// Bus busy / 总线忙
    Busy,
    /// Timeout / 超时
    Timeout,
    /// Hardware error / 硬件错误
    HardwareError,
    /// Feature not supported / 功能不支持
    NotSupported,
    /// DMA error / DMA 错误
    DmaError,
    /// Invalid mode / 无效模式
    InvalidMode,
    /// Invalid frequency / 无效频率
    InvalidFrequency,
    /// Chip select error / 片选错误
    CsError,
    /// Overrun error / 溢出错误
    Overrun,
    /// Underrun error / 下溢错误
    Underrun,
    /// Frame format error / 帧格式错误
    FrameFormatError,
}

impl core::fmt::Display for SpiError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SpiError::InvalidConfig => write!(f, "Invalid configuration"),
            SpiError::NotInitialized => write!(f, "Device not initialized"),
            SpiError::AlreadyInitialized => write!(f, "Device already initialized"),
            SpiError::BufferTooSmall => write!(f, "Buffer too small"),
            SpiError::Busy => write!(f, "Bus busy"),
            SpiError::Timeout => write!(f, "Timeout"),
            SpiError::HardwareError => write!(f, "Hardware error"),
            SpiError::NotSupported => write!(f, "Feature not supported"),
            SpiError::DmaError => write!(f, "DMA error"),
            SpiError::InvalidMode => write!(f, "Invalid mode"),
            SpiError::InvalidFrequency => write!(f, "Invalid frequency"),
            SpiError::CsError => write!(f, "Chip select error"),
            SpiError::Overrun => write!(f, "Overrun error"),
            SpiError::Underrun => write!(f, "Underrun error"),
            SpiError::FrameFormatError => write!(f, "Frame format error"),
        }
    }
}
