//! Timer Driver
//! 定时器驱动

/// Timer error types / 定时器错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerError {
    NotInitialized,
    AlreadyRunning,
    NotRunning,
    InvalidChannel,
}

impl core::fmt::Display for TimerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TimerError::NotInitialized => write!(f, "Not initialized"),
            TimerError::AlreadyRunning => write!(f, "Already running"),
            TimerError::NotRunning => write!(f, "Not running"),
            TimerError::InvalidChannel => write!(f, "Invalid channel"),
        }
    }
}

/// Timer mode / 定时器模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimerMode {
    #[default]
    Periodic,
    OneShot,
}

/// Timer configuration / 定时器配置
#[derive(Debug, Clone, Default)]
pub struct TimerConfig {
    pub mode: TimerMode,
    pub period_ns: u32,
}

/// Timer driver trait / 定时器驱动特征
pub trait TimerDriver {
    fn init(&mut self, config: &TimerConfig) -> Result<(), TimerError>;
    fn start(&mut self) -> Result<(), TimerError>;
    fn stop(&mut self) -> Result<(), TimerError>;
    fn get_counter(&self) -> Result<u32, TimerError>;
}
