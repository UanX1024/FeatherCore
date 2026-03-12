//! Timer/Counter Driver Traits
//! 定时器/计数器驱动特征定义
//!
//! This module defines the traits that all timer/counter drivers must implement.
//! It provides a common interface for timer operations independent of
//! the underlying hardware platform.
//! 该模块定义了所有定时器/计数器驱动必须实现的特征。它提供了与底层硬件平台无关的
//! 通用定时器操作接口。

use crate::driver::timer::{TimerAlarmConfig, TimerChannel, TimerConfig, TimerError};

/// Timer driver state / 定时器驱动状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    /// Not ready / 未就绪
    NotReady,
    /// Ready / 就绪
    Ready,
    /// Running / 运行中
    Running,
    /// Stopped / 已停止
    Stopped,
    /// Error / 错误
    Error,
}

/// Alarm callback type / 闹钟回调类型
pub type TimerAlarmCallback = fn(usize);

/// Timer driver trait that must be implemented by all platform-specific drivers
/// 定时器驱动特征，所有平台特定的驱动都必须实现
pub trait TimerDriver {
    /// Initialize the timer driver with the given configuration
    /// 使用给定配置初始化定时器驱动
    fn init(&mut self, config: &TimerConfig) -> Result<(), TimerError>;

    /// Start the timer / 启动定时器
    fn start(&mut self) -> Result<(), TimerError>;

    /// Stop the timer / 停止定时器
    fn stop(&mut self) -> Result<(), TimerError>;

    /// Reset the timer / 重设定时器
    fn reset(&mut self) -> Result<(), TimerError>;

    /// Get current counter value / 获取当前计数器值
    fn get_value(&self) -> Result<u32, TimerError>;

    /// Set counter value / 设置计数器值
    fn set_value(&mut self, value: u32) -> Result<(), TimerError>;

    /// Set alarm for a channel / 设置通道闹钟
    fn set_alarm(&mut self, channel: TimerChannel, config: TimerAlarmConfig) -> Result<(), TimerError>;

    /// Cancel alarm for a channel / 取消通道闹钟
    fn cancel_alarm(&mut self, channel: TimerChannel) -> Result<(), TimerError>;

    /// Get timer frequency / 获取定时器频率
    fn get_frequency(&self) -> Result<u32, TimerError>;

    /// Get maximum top value / 获取最大顶部值
    fn get_max_top_value(&self) -> Result<u32, TimerError>;

    /// Set top value / 设置顶部值
    fn set_top_value(&mut self, top: u32) -> Result<(), TimerError>;

    /// Get current timer state / 获取当前定时器状态
    fn state(&self) -> TimerState;

    /// Check if timer is running / 检查定时器是否运行
    fn is_running(&self) -> bool {
        self.state() == TimerState::Running
    }

    /// Enable the timer peripheral / 使能定时器外设
    fn enable(&mut self) -> Result<(), TimerError>;

    /// Disable the timer peripheral / 禁用定时器外设
    fn disable(&mut self) -> Result<(), TimerError>;
}
