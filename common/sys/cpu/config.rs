//! CPU Configuration
//! CPU 配置

/// CPU configuration structure
/// CPU 配置结构
#[derive(Debug, Clone, Default)]
pub struct CpuConfig {
    /// CPU cores configuration
    /// CPU 核心配置
    pub cores: Vec<CpuCoreConfig>,
    /// Cache configuration
    /// 缓存配置
    pub cache: CacheConfig,
    /// Memory map configuration
    /// 内存映射配置
    pub memory_map: MemoryMapConfig,
    /// Frequency configuration
    /// 频率配置
    pub frequency: FrequencyConfig,
    /// Features configuration
    /// 特性配置
    pub features: CpuFeatures,
}

/// CPU core configuration
/// CPU 核心配置
#[derive(Debug, Clone)]
pub struct CpuCoreConfig {
    /// Core ID
    /// 核心 ID
    pub id: u32,
    /// Core type (e.g., "cortex-m4")
    /// 核心类型（如 "cortex-m4"）
    pub core_type: &'static str,
    /// Architecture (e.g., "arm")
    /// 架构（如 "arm"）
    pub architecture: &'static str,
    /// Revision
    /// 版本
    pub revision: &'static str,
    /// Features
    /// 特性
    pub features: Vec<&'static str>,
    /// Maximum frequency in Hz
    /// 最大频率（Hz）
    pub max_frequency: u32,
    /// Default frequency in Hz
    /// 默认频率（Hz）
    pub default_frequency: u32,
}

/// Cache configuration
/// 缓存配置
#[derive(Debug, Clone, Default)]
pub struct CacheConfig {
    /// L1 instruction cache size in bytes
    /// L1 指令缓存大小（字节）
    pub l1_instruction_cache: usize,
    /// L1 data cache size in bytes
    /// L1 数据缓存大小（字节）
    pub l1_data_cache: usize,
    /// L2 cache size in bytes
    /// L2 缓存大小（字节）
    pub l2_cache: usize,
    /// L3 cache size in bytes
    /// L3 缓存大小（字节）
    pub l3_cache: usize,
    /// Cache line size in bytes
    /// 缓存行大小（字节）
    pub cache_line_size: usize,
    /// Enable cache
    /// 启用缓存
    pub enable_cache: bool,
}

/// Memory map configuration
/// 内存映射配置
#[derive(Debug, Clone, Default)]
pub struct MemoryMapConfig {
    /// Memory regions
    /// 内存区域
    pub regions: Vec<MemoryRegionConfig>,
    /// Enable memory protection
    /// 启用内存保护
    pub enable_memory_protection: bool,
}

/// Memory region configuration
/// 内存区域配置
#[derive(Debug, Clone)]
pub struct MemoryRegionConfig {
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

/// Frequency configuration
/// 频率配置
#[derive(Debug, Clone, Default)]
pub struct FrequencyConfig {
    /// Default frequency in Hz
    /// 默认频率（Hz）
    pub default_frequency: u32,
    /// Maximum frequency in Hz
    /// 最大频率（Hz）
    pub max_frequency: u32,
    /// Minimum frequency in Hz
    /// 最小频率（Hz）
    pub min_frequency: u32,
    /// Available frequencies in Hz
    /// 可用频率（Hz）
    pub available_frequencies: Vec<u32>,
    /// Enable dynamic frequency scaling
    /// 启用动态频率调整
    pub enable_dynamic_scaling: bool,
}

/// CPU features configuration
/// CPU 特性配置
#[derive(Debug, Clone, Default)]
pub struct CpuFeatures {
    /// Enable FPU
    /// 启用 FPU
    pub enable_fpu: bool,
    /// Enable MPU
    /// 启用 MPU
    pub enable_mpu: bool,
    /// Enable FPU
    /// 启用 FPU
    pub enable_nvic: bool,
    /// Enable instruction cache
    /// 启用指令缓存
    pub enable_icache: bool,
    /// Enable data cache
    /// 启用数据缓存
    pub enable_dcache: bool,
    /// Enable branch prediction
    /// 启用分支预测
    pub enable_branch_prediction: bool,
    /// Enable DSP instructions
    /// 启用 DSP 指令
    pub enable_dsp: bool,
    /// Enable SIMD instructions
    /// 启用 SIMD 指令
    pub enable_simd: bool,
    /// Enable security extension
    /// 启用安全扩展
    pub enable_security: bool,
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
