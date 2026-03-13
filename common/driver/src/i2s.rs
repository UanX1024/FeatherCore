//! I2S Driver
//! I2S 驱动

/// I2S error types / I2S 错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2sError {
    NotInitialized,
    BusBusy,
    Underrun,
    Overrun,
}

impl core::fmt::Display for I2sError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            I2sError::NotInitialized => write!(f, "Not initialized"),
            I2sError::BusBusy => write!(f, "Bus busy"),
            I2sError::Underrun => write!(f, "Underrun"),
            I2sError::Overrun => write!(f, "Overrun"),
        }
    }
}

/// I2S mode / I2S 模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum I2sMode {
    #[default]
    Master,
    Slave,
}

/// I2S configuration / I2S 配置
#[derive(Debug, Clone, Default)]
pub struct I2sConfig {
    pub mode: I2sMode,
    pub sample_rate: u32,
}

/// I2S driver trait / I2S 驱动特征
pub trait I2sDriver {
    fn init(&mut self, config: &I2sConfig) -> Result<(), I2sError>;
    fn write(&mut self, data: &[u8]) -> Result<usize, I2sError>;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, I2sError>;
}
