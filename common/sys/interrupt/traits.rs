//! Interrupt Traits
//! 中断接口定义

/// Interrupt driver trait
/// 中断驱动接口
pub trait InterruptDriver {
    /// Initialize the interrupt system
    /// 初始化中断系统
    fn init(&self) -> Result<(), InterruptError>;
    
    /// Enable an interrupt
    /// 启用中断
    fn enable(&self, irq: u32) -> Result<(), InterruptError>;
    
    /// Disable an interrupt
    /// 禁用中断
    fn disable(&self, irq: u32) -> Result<(), InterruptError>;
    
    /// Set interrupt priority
    /// 设置中断优先级
    fn set_priority(&self, irq: u32, priority: u8) -> Result<(), InterruptError>;
    
    /// Register an interrupt handler
    /// 注册中断处理函数
    fn register_handler(&self, irq: u32, handler: InterruptHandler) -> Result<(), InterruptError>;
    
    /// Unregister an interrupt handler
    /// 注销中断处理函数
    fn unregister_handler(&self, irq: u32) -> Result<(), InterruptError>;
    
    /// Get current interrupt status
    /// 获取当前中断状态
    fn get_status(&self, irq: u32) -> bool;
    
    /// Clear interrupt status
    /// 清除中断状态
    fn clear_status(&self, irq: u32) -> Result<(), InterruptError>;
}

/// Interrupt handler type
/// 中断处理函数类型
pub type InterruptHandler = fn();

/// Interrupt controller trait
/// 中断控制器接口
pub trait InterruptController {
    /// Get the number of supported interrupts
    /// 获取支持的中断数量
    fn num_interrupts(&self) -> u32;
    
    /// Get the maximum priority level
    /// 获取最大优先级级别
    fn max_priority(&self) -> u8;
    
    /// Trigger a software interrupt
    /// 触发软件中断
    fn trigger_software_interrupt(&self, irq: u32) -> Result<(), InterruptError>;
}
