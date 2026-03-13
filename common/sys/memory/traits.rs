//! Memory Traits
//! 内存接口定义

/// Memory driver trait
/// 内存驱动接口
pub trait MemoryDriver {
    /// Initialize the memory system
    /// 初始化内存系统
    fn init(&self) -> Result<(), MemoryError>;
    
    /// Allocate memory
    /// 分配内存
    fn allocate(&self, size: usize, alignment: usize) -> Result<*mut u8, MemoryError>;
    
    /// Free memory
    /// 释放内存
    fn free(&self, ptr: *mut u8) -> Result<(), MemoryError>;
    
    /// Get total memory size
    /// 获取总内存大小
    fn total_memory(&self) -> usize;
    
    /// Get free memory size
    /// 获取可用内存大小
    fn free_memory(&self) -> usize;
}

/// Memory region trait
/// 内存区域接口
pub trait MemoryRegion {
    /// Get base address of the memory region
    /// 获取内存区域的基地址
    fn base_address(&self) -> usize;
    
    /// Get size of the memory region
    /// 获取内存区域的大小
    fn size(&self) -> usize;
    
    /// Check if an address is within this region
    /// 检查地址是否在该区域内
    fn contains(&self, address: usize) -> bool;
}
