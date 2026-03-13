//! Interrupt Error Types
//! 中断错误类型

/// Interrupt-related errors
/// 中断相关错误
#[derive(Debug, PartialEq, Eq)]
pub enum InterruptError {
    /// Invalid interrupt number
    /// 无效的中断号
    InvalidInterrupt,
    /// Interrupt initialization failed
    /// 中断初始化失败
    InitializationFailed,
    /// Interrupt handler registration failed
    /// 中断处理函数注册失败
    RegistrationFailed,
    /// Invalid priority level
    /// 无效的优先级级别
    InvalidPriority,
    /// Interrupt not found
    /// 中断未找到
    InterruptNotFound,
}

impl core::fmt::Display for InterruptError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InterruptError::InvalidInterrupt => write!(f, "Invalid interrupt number"),
            InterruptError::InitializationFailed => write!(f, "Interrupt initialization failed"),
            InterruptError::RegistrationFailed => write!(f, "Interrupt handler registration failed"),
            InterruptError::InvalidPriority => write!(f, "Invalid priority level"),
            InterruptError::InterruptNotFound => write!(f, "Interrupt not found"),
        }
    }
}
