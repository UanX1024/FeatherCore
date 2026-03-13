//! SPI Driver
//! SPI 驱动

/// SPI error types / SPI 错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiError {
    NotInitialized,
    BusBusy,
    Timeout,
}

impl core::fmt::Display for SpiError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SpiError::NotInitialized => write!(f, "Not initialized"),
            SpiError::BusBusy => write!(f, "Bus busy"),
            SpiError::Timeout => write!(f, "Timeout"),
        }
    }
}

/// SPI mode / SPI 模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpiMode {
    #[default]
    Master,
    Slave,
}

/// SPI configuration / SPI 配置
#[derive(Debug, Clone, Default)]
pub struct SpiConfig {
    pub mode: SpiMode,
    pub frequency: u32,
}

/// SPI driver trait / SPI 驱动特征
pub trait SpiDriver {
    fn init(&mut self, config: &SpiConfig) -> Result<(), SpiError>;
    fn transfer(&mut self, tx: &[u8], rx: &mut [u8]) -> Result<usize, SpiError>;
}
