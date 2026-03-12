//! Timer/Counter Driver Error Types
//! 定时器/计数器驱动错误类型
//!
//! This module defines the error types used by the timer/counter driver.
//! 该模块定义了定时器/计数器驱动使用的错误类型。

/// Timer driver error types
/// 定时器驱动错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerError {
    /// Invalid configuration / 无效配置
    InvalidConfig,
    /// Device not initialized / 设备未初始化
    NotInitialized,
    /// Device already initialized / 设备已初始化
    AlreadyInitialized,
    /// Timer not running / 定时器未运行
    NotRunning,
    /// Timer already running / 定时器已在运行
    AlreadyRunning,
    /// Channel invalid / 通道无效
    InvalidChannel,
    /// Channel already in use / 通道已使用
    ChannelInUse,
    /// Alarm already set / 闹钟已设置
    AlarmSet,
    /// Alarm not set / 闹钟未设置
    AlarmNotSet,
    /// Callback error / 回调错误
    CallbackError,
    /// Timeout / 超时
    Timeout,
    /// Hardware error / 硬件错误
    HardwareError,
    /// Feature not supported / 功能不支持
    NotSupported,
    /// DMA error / DMA 错误
    DmaError,
    /// Counter overflow / 计数器溢出
    Overflow,
}

impl core::fmt::Display for TimerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TimerError::InvalidConfig => write!(f, "Invalid configuration"),
            TimerError::NotInitialized => write!(f, "Device not initialized"),
            TimerError::AlreadyInitialized => write!(f, "Device already initialized"),
            TimerError::NotRunning => write!(f, "Timer not running"),
            TimerError::AlreadyRunning => write!(f, "Timer already running"),
            TimerError::InvalidChannel => write!(f, "Invalid channel"),
            TimerError::ChannelInUse => write!(f, "Channel already in use"),
            TimerError::AlarmSet => write!(f, "Alarm already set"),
            TimerError::AlarmNotSet => write!(f, "Alarm not set"),
            TimerError::CallbackError => write!(f, "Callback error"),
            TimerError::Timeout => write!(f, "Timeout"),
            TimerError::HardwareError => write!(f, "Hardware error"),
            TimerError::NotSupported => write!(f, "Feature not supported"),
            TimerError::DmaError => write!(f, "DMA error"),
            TimerError::Overflow => write!(f, "Counter overflow"),
        }
    }
}
