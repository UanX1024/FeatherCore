//! CPU Error Types
//! CPU 错误类型

/// CPU-related errors
/// CPU 相关错误
#[derive(Debug, PartialEq, Eq)]
pub enum CpuError {
    /// Invalid core ID
    /// 无效的核心 ID
    InvalidCoreId,
    /// CPU initialization failed
    /// CPU 初始化失败
    InitializationFailed,
    /// Invalid frequency
    /// 无效的频率
    InvalidFrequency,
    /// Operation not supported
    /// 操作不支持
    NotSupported,
    /// Permission denied
    /// 权限被拒绝
    PermissionDenied,
    /// Hardware error
    /// 硬件错误
    HardwareError,
}

impl core::fmt::Display for CpuError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CpuError::InvalidCoreId => write!(f, "Invalid core ID"),
            CpuError::InitializationFailed => write!(f, "CPU initialization failed"),
            CpuError::InvalidFrequency => write!(f, "Invalid frequency"),
            CpuError::NotSupported => write!(f, "Operation not supported"),
            CpuError::PermissionDenied => write!(f, "Permission denied"),
            CpuError::HardwareError => write!(f, "Hardware error"),
        }
    }
}
