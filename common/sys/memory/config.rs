//! Memory Configuration
//! 内存配置

/// Memory configuration structure
/// 内存配置结构
#[derive(Debug, Clone, Default)]
pub struct MemoryConfig {
    /// RAM regions
    /// RAM 区域
    pub ram_regions: Vec<RamRegion>,
    /// Flash regions
    /// Flash 区域
    pub flash_regions: Vec<FlashRegion>,
    /// Heap configuration
    /// 堆配置
    pub heap_config: HeapConfig,
}

/// RAM region configuration
/// RAM 区域配置
#[derive(Debug, Clone)]
pub struct RamRegion {
    /// Base address
    /// 基地址
    pub base_address: usize,
    /// Size in bytes
    /// 大小（字节）
    pub size: usize,
    /// Region name
    /// 区域名称
    pub name: &'static str,
    /// Whether this region is cacheable
    /// 是否可缓存
    pub cacheable: bool,
}

/// Flash region configuration
/// Flash 区域配置
#[derive(Debug, Clone)]
pub struct FlashRegion {
    /// Base address
    /// 基地址
    pub base_address: usize,
    /// Size in bytes
    /// 大小（字节）
    pub size: usize,
    /// Region name
    /// 区域名称
    pub name: &'static str,
    /// Sector size in bytes
    /// 扇区大小（字节）
    pub sector_size: usize,
}

/// Heap configuration
/// 堆配置
#[derive(Debug, Clone, Default)]
pub struct HeapConfig {
    /// Heap size in bytes
    /// 堆大小（字节）
    pub size: usize,
    /// Minimum allocation alignment
    /// 最小分配对齐
    pub min_alignment: usize,
    /// Whether to enable heap statistics
    /// 是否启用堆统计
    pub enable_statistics: bool,
}
