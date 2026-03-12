//! I2S Driver Error Types
//! I2S 驱动错误类型
//!
//! This module defines the error types used by the I2S driver.
//! 该模块定义了 I2S 驱动使用的错误类型。

/// I2S driver error types
/// I2S 驱动错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2sError {
    /// Invalid configuration / 无效配置
    InvalidConfig,
    /// Device not initialized / 设备未初始化
    NotInitialized,
    /// Device already initialized / 设备已初始化
    AlreadyInitialized,
    /// Buffer underrun (TX) / 缓冲区下溢 (发送)
    Underrun,
    /// Buffer overrun (RX) / 缓冲区上溢 (接收)
    Overrun,
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
    /// Invalid format / 无效格式
    InvalidFormat,
    /// Clock error / 时钟错误
    ClockError,
}

impl core::fmt::Display for I2sError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            I2sError::InvalidConfig => write!(f, "Invalid configuration"),
            I2sError::NotInitialized => write!(f, "Device not initialized"),
            I2sError::AlreadyInitialized => write!(f, "Device already initialized"),
            I2sError::Underrun => write!(f, "Buffer underrun"),
            I2sError::Overrun => write!(f, "Buffer overrun"),
            I2sError::Busy => write!(f, "Bus busy"),
            I2sError::Timeout => write!(f, "Timeout"),
            I2sError::HardwareError => write!(f, "Hardware error"),
            I2sError::NotSupported => write!(f, "Feature not supported"),
            I2sError::DmaError => write!(f, "DMA error"),
            I2sError::InvalidFormat => write!(f, "Invalid format"),
            I2sError::ClockError => write!(f, "Clock error"),
        }
    }
}
