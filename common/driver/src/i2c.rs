//! I2C Driver
//! I2C 驱动

/// I2C error types / I2C 错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2cError {
    BusBusy,
    Timeout,
    InvalidAddress,
    InvalidData,
}

impl core::fmt::Display for I2cError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            I2cError::BusBusy => write!(f, "Bus busy"),
            I2cError::Timeout => write!(f, "Timeout"),
            I2cError::InvalidAddress => write!(f, "Invalid address"),
            I2cError::InvalidData => write!(f, "Invalid data"),
        }
    }
}

/// I2C speed / I2C 速度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum I2cSpeed {
    #[default]
    Standard,
    Fast,
    FastPlus,
}

/// I2C configuration / I2C 配置
#[derive(Debug, Clone, Default)]
pub struct I2cConfig {
    pub speed: I2cSpeed,
    pub address_bits: u8,
}

/// I2C driver trait / I2C 驱动特征
pub trait I2cDriver {
    fn init(&mut self, config: &I2cConfig) -> Result<(), I2cError>;
    fn write(&mut self, addr: u8, data: &[u8]) -> Result<(), I2cError>;
    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), I2cError>;
}
