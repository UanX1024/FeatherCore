//! CPU Traits
//! CPU 接口定义

/// CPU driver trait
/// CPU 驱动接口
pub trait CpuDriver {
    /// Initialize the CPU system
    /// 初始化 CPU 系统
    fn init(&self) -> Result<(), CpuError>;
    
    /// Get the number of CPU cores
    /// 获取 CPU 核心数量
    fn num_cores(&self) -> u32;
    
    /// Get CPU core information
    /// 获取 CPU 核心信息
    fn get_core(&self, core_id: u32) -> Result<&dyn CpuCore, CpuError>;
    
    /// Get CPU cache information
    /// 获取 CPU 缓存信息
    fn get_cache(&self) -> &dyn CpuCache;
    
    /// Get memory map information
    /// 获取内存映射信息
    fn get_memory_map(&self) -> &dyn MemoryMap;
    
    /// Set CPU frequency
    /// 设置 CPU 频率
    fn set_frequency(&self, freq: u32) -> Result<(), CpuError>;
    
    /// Get current CPU frequency
    /// 获取当前 CPU 频率
    fn get_frequency(&self) -> u32;
    
    /// Enable interrupts
    /// 启用中断
    fn enable_interrupts(&self);
    
    /// Disable interrupts
    /// 禁用中断
    fn disable_interrupts(&self);
    
    /// Get interrupts status
    /// 获取中断状态
    fn get_interrupts_status(&self) -> bool;
    
    /// Put CPU to sleep
    /// 使 CPU 进入睡眠状态
    fn sleep(&self);
    
    /// Reset the system
    /// 重置系统
    fn reset(&self);
}

/// CPU core trait
/// CPU 核心接口
pub trait CpuCore {
    /// Get core ID
    /// 获取核心 ID
    fn id(&self) -> u32;
    
    /// Get core type
    /// 获取核心类型
    fn core_type(&self) -> &str;
    
    /// Get core architecture
    /// 获取核心架构
    fn architecture(&self) -> &str;
    
    /// Get core revision
    /// 获取核心版本
    fn revision(&self) -> &str;
    
    /// Get core features
    /// 获取核心特性
    fn features(&self) -> &[&str];
    
    /// Get core current frequency
    /// 获取核心当前频率
    fn current_frequency(&self) -> u32;
    
    /// Get core maximum frequency
    /// 获取核心最大频率
    fn max_frequency(&self) -> u32;
}

/// CPU cache trait
/// CPU 缓存接口
pub trait CpuCache {
    /// Get L1 instruction cache size in bytes
    /// 获取 L1 指令缓存大小（字节）
    fn l1_instruction_cache_size(&self) -> usize;
    
    /// Get L1 data cache size in bytes
    /// 获取 L1 数据缓存大小（字节）
    fn l1_data_cache_size(&self) -> usize;
    
    /// Get L2 cache size in bytes
    /// 获取 L2 缓存大小（字节）
    fn l2_cache_size(&self) -> usize;
    
    /// Get L3 cache size in bytes
    /// 获取 L3 缓存大小（字节）
    fn l3_cache_size(&self) -> usize;
    
    /// Get cache line size in bytes
    /// 获取缓存行大小（字节）
    fn cache_line_size(&self) -> usize;
    
    /// Invalidate cache
    /// 使缓存失效
    fn invalidate(&self);
    
    /// Clean cache
    /// 清理缓存
    fn clean(&self);
    
    /// Clean and invalidate cache
    /// 清理并使缓存失效
    fn clean_invalidate(&self);
}

/// Memory map trait
/// 内存映射接口
pub trait MemoryMap {
    /// Get memory regions
    /// 获取内存区域
    fn regions(&self) -> &[MemoryRegion];
    
    /// Find memory region by address
    /// 根据地址查找内存区域
    fn find_region(&self, address: usize) -> Option<&MemoryRegion>;
    
    /// Get total memory size
    /// 获取总内存大小
    fn total_memory(&self) -> usize;
}

/// Memory region structure
/// 内存区域结构
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    /// Region name
    /// 区域名称
    pub name: &'static str,
    /// Base address
    /// 基地址
    pub base_address: usize,
    /// Size in bytes
    /// 大小（字节）
    pub size: usize,
    /// Region type
    /// 区域类型
    pub region_type: MemoryRegionType,
    /// Access permissions
    /// 访问权限
    pub permissions: MemoryPermissions,
}

/// Memory region type
/// 内存区域类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegionType {
    /// RAM
    Ram,
    /// Flash
    Flash,
    /// Peripheral
    Peripheral,
    /// Device
    Device,
    /// Reserved
    Reserved,
}

/// Memory permissions
/// 内存访问权限
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryPermissions {
    /// Read permission
    pub read: bool,
    /// Write permission
    pub write: bool,
    /// Execute permission
    pub execute: bool,
    /// Cacheable
    pub cacheable: bool,
    /// Bufferable
    pub bufferable: bool,
}
