//! Clock Error Types
//! 时钟错误类型

/// Clock-related errors
/// 时钟相关错误
#[derive(Debug, PartialEq, Eq)]
pub enum ClockError {
    /// Invalid clock frequency
    /// 无效的时钟频率
    InvalidFrequency,
    /// Clock initialization failed
    /// 时钟初始化失败
    InitializationFailed,
    /// Clock configuration failed
    /// 时钟配置失败
    ConfigurationFailed,
    /// Peripheral not found
    /// 外设未找到
    PeripheralNotFound,
}

impl core::fmt::Display for ClockError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ClockError::InvalidFrequency => write!(f, "Invalid clock frequency"),
            ClockError::InitializationFailed => write!(f, "Clock initialization failed"),
            ClockError::ConfigurationFailed => write!(f, "Clock configuration failed"),
            ClockError::PeripheralNotFound => write!(f, "Peripheral not found"),
        }
    }
}
