//! GPIO Driver
//! GPIO 驱动

/// GPIO error types / GPIO 错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioError {
    InvalidPin,
    InvalidMode,
    NotInitialized,
    AlreadyInitialized,
}

impl core::fmt::Display for GpioError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GpioError::InvalidPin => write!(f, "Invalid pin"),
            GpioError::InvalidMode => write!(f, "Invalid mode"),
            GpioError::NotInitialized => write!(f, "Not initialized"),
            GpioError::AlreadyInitialized => write!(f, "Already initialized"),
        }
    }
}

/// GPIO pin mode / GPIO 引脚模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GpioMode {
    Input,
    Output,
    #[default]
    AltFunction,
    Analog,
}

/// GPIO speed / GPIO 速度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GpioSpeed {
    Low,
    Medium,
    #[default]
    High,
    VeryHigh,
}

/// GPIO pull-up/pull-down / GPIO 上拉/下拉
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GpioPull {
    #[default]
    NoPull,
    PullUp,
    PullDown,
}

/// GPIO configuration / GPIO 配置
#[derive(Debug, Clone, Default)]
pub struct GpioConfig {
    pub mode: GpioMode,
    pub speed: GpioSpeed,
    pub pull: GpioPull,
}

/// GPIO driver trait / GPIO 驱动特征
pub trait GpioDriver {
    fn init(&mut self) -> Result<(), GpioError>;
    fn set_mode(&mut self, pin: u8, mode: GpioMode) -> Result<(), GpioError>;
    fn write(&mut self, pin: u8, value: bool) -> Result<(), GpioError>;
    fn read(&self, pin: u8) -> Result<bool, GpioError>;
}
