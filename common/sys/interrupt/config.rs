//! Interrupt Configuration
//! 中断配置

/// Interrupt configuration structure
/// 中断配置结构
#[derive(Debug, Clone, Default)]
pub struct InterruptConfig {
    /// Interrupt controller configuration
    /// 中断控制器配置
    pub controller: InterruptControllerConfig,
    /// Interrupt priorities
    /// 中断优先级
    pub priorities: Vec<InterruptPriority>,
    /// Whether to enable nested interrupts
    /// 是否启用嵌套中断
    pub enable_nested_interrupts: bool,
}

/// Interrupt controller configuration
/// 中断控制器配置
#[derive(Debug, Clone, Default)]
pub struct InterruptControllerConfig {
    /// Number of interrupt lines
    /// 中断线数量
    pub num_interrupts: u32,
    /// Maximum priority level
    /// 最大优先级级别
    pub max_priority: u8,
    /// Priority bits
    /// 优先级位数
    pub priority_bits: u8,
    /// Whether to enable software interrupts
    /// 是否启用软件中断
    pub enable_software_interrupts: bool,
}

/// Interrupt priority configuration
/// 中断优先级配置
#[derive(Debug, Clone)]
pub struct InterruptPriority {
    /// Interrupt number
    /// 中断号
    pub irq: u32,
    /// Priority level (0 = highest)
    /// 优先级级别（0 = 最高）
    pub priority: u8,
}

/// Interrupt trigger mode
/// 中断触发模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptTrigger {
    /// Level-sensitive interrupt
    /// 电平触发
    Level,
    /// Edge-sensitive interrupt
    /// 边沿触发
    Edge,
    /// Both level and edge sensitive
    /// 电平边沿都触发
    Both,
}
