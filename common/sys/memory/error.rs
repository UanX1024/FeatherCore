//! Memory Error Types
//! 内存错误类型

/// Memory-related errors
/// 内存相关错误
#[derive(Debug, PartialEq, Eq)]
pub enum MemoryError {
    /// Out of memory
    /// 内存不足
    OutOfMemory,
    /// Invalid memory address
    /// 无效的内存地址
    InvalidAddress,
    /// Memory initialization failed
    /// 内存初始化失败
    InitializationFailed,
    /// Invalid allocation size
    /// 无效的分配大小
    InvalidSize,
    /// Invalid alignment
    /// 无效的对齐方式
    InvalidAlignment,
}

impl core::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MemoryError::OutOfMemory => write!(f, "Out of memory"),
            MemoryError::InvalidAddress => write!(f, "Invalid memory address"),
            MemoryError::InitializationFailed => write!(f, "Memory initialization failed"),
            MemoryError::InvalidSize => write!(f, "Invalid allocation size"),
            MemoryError::InvalidAlignment => write!(f, "Invalid alignment"),
        }
    }
}
