//! Timer/Counter Driver Configuration
//! 定时器/计数器驱动配置
//!
//! This module defines the configuration structures for timer/counter driver.
//! 该模块定义了定时器/计数器驱动的配置结构。

/// Timer/Counter mode / 定时器/计数器模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum TimerMode {
    /// Free-running counter / 自由运行计数器
    #[default]
    FreeRunning,
    /// User-defined period / 用户定义周期
    Periodic,
    /// One-shot mode / 单次模式
    OneShot,
    /// PWM mode / PWM 模式
    Pwm,
}

/// Timer/Counter clock source / 定时器/计数器时钟源
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum TimerClockSource {
    /// Internal clock / 内部时钟
    #[default]
    Internal,
    /// External clock / 外部时钟
    External,
    /// Trigger clock / 触发时钟
    Trigger,
}

/// Timer/Counter channel / 定时器/计数器通道
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum TimerChannel {
    /// Channel 0 / 通道 0
    #[default]
    Channel0 = 0,
    /// Channel 1 / 通道 1
    Channel1 = 1,
    /// Channel 2 / 通道 2
    Channel2 = 2,
    /// Channel 3 / 通道 3
    Channel3 = 3,
}

impl TimerChannel {
    /// Get channel number / 获取通道编号
    pub fn number(&self) -> u8 {
        *self as u8
    }
}

/// Timer/Counter prescaler / 定时器/计数器预分频器
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TimerPrescaler {
    /// Prescaler value / 预分频值
    pub value: u16,
    /// Prescaler factor (2^factor) / 预分频因子 (2^factor)
    pub factor: u8,
}

impl TimerPrescaler {
    /// Create new prescaler / 创建新预分频器
    pub fn new(value: u16) -> Self {
        Self {
            value,
            factor: 0,
        }
    }

    /// Calculate actual frequency division / 计算实际频率分频
    pub fn division(&self) -> u32 {
        (self.value as u32) * (1 << self.factor)
    }
}

/// Timer/Counter alarm configuration / 定时器/计数器闹钟配置
#[derive(Debug, Clone, Default)]
pub struct TimerAlarmConfig {
    /// Channel / 通道
    pub channel: TimerChannel,
    /// Alarm ticks / 闹钟 ticks
    pub ticks: u32,
    /// Auto reload / 自动重载
    pub auto_reload: bool,
    /// Callback function / 回调函数
    pub callback: Option<fn(usize)>,
    /// User data / 用户数据
    pub user_data: usize,
}

/// Timer/Counter configuration structure / 定时器/计数器配置结构体
#[derive(Debug, Clone, Default)]
pub struct TimerConfig {
    /// Timer mode / 定时器模式
    pub mode: TimerMode,
    /// Clock source / 时钟源
    pub clock_source: TimerClockSource,
    /// Prescaler / 预分频器
    pub prescaler: TimerPrescaler,
    /// Counter period (auto-reload value) / 计数器周期 (自动重载值)
    pub period: u32,
    /// Counter direction / 计数器方向
    pub direction: TimerDirection,
    /// Enable interrupt / 使能中断
    pub interrupt_enabled: bool,
    /// Enable DMA / 使能 DMA
    pub dma_enabled: bool,
    /// DMA buffer size / DMA 缓冲区大小
    pub dma_buffer_size: usize,
}

/// Timer/Counter direction / 定时器/计数器方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum TimerDirection {
    /// Count up / 向上计数
    #[default]
    Up,
    /// Count down / 向下计数
    Down,
}

impl TimerConfig {
    /// Create a new default timer configuration / 创建默认定时器配置
    pub fn new() -> Self {
        Self::default()
    }

    /// Create configuration for free-running mode / 创建自由运行模式配置
    pub fn free_running() -> Self {
        let mut config = Self::default();
        config.mode = TimerMode::FreeRunning;
        config
    }

    /// Create configuration for periodic mode / 创建周期模式配置
    pub fn periodic() -> Self {
        let mut config = Self::default();
        config.mode = TimerMode::Periodic;
        config
    }

    /// Create configuration for one-shot mode / 创建单次模式配置
    pub fn one_shot() -> Self {
        let mut config = Self::default();
        config.mode = TimerMode::OneShot;
        config
    }

    /// Set prescaler / 设置预分频器
    pub fn with_prescaler(mut self, value: u16, factor: u8) -> Self {
        self.prescaler = TimerPrescaler { value, factor };
        self
    }

    /// Set period / 设置周期
    pub fn with_period(mut self, period: u32) -> Self {
        self.period = period;
        self
    }

    /// Set clock source / 设置时钟源
    pub fn with_clock_source(mut self, source: TimerClockSource) -> Self {
        self.clock_source = source;
        self
    }

    /// Set direction / 设置方向
    pub fn with_direction(mut self, direction: TimerDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Enable interrupt / 使能中断
    pub fn with_interrupt(mut self, enabled: bool) -> Self {
        self.interrupt_enabled = enabled;
        self
    }

    /// Enable DMA / 使能 DMA
    pub fn with_dma(mut self, enabled: bool, buffer_size: usize) -> Self {
        self.dma_enabled = enabled;
        self.dma_buffer_size = buffer_size;
        self
    }
}
