//! Serial Driver
//! 串口驱动

/// Serial error types / 串口错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerialError {
    NotInitialized,
    TxBusy,
    RxBusy,
    Timeout,
}

impl core::fmt::Display for SerialError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SerialError::NotInitialized => write!(f, "Not initialized"),
            SerialError::TxBusy => write!(f, "Transmitter busy"),
            SerialError::RxBusy => write!(f, "Receiver busy"),
            SerialError::Timeout => write!(f, "Timeout"),
        }
    }
}

/// Serial configuration / 串口配置
#[derive(Debug, Clone, Default)]
pub struct SerialConfig {
    pub baud_rate: u32,
    pub data_bits: u8,
    pub stop_bits: u8,
}

/// Serial driver trait / 串口驱动特征
pub trait SerialDriver {
    fn init(&mut self, config: &SerialConfig) -> Result<(), SerialError>;
    fn write(&mut self, data: &[u8]) -> Result<usize, SerialError>;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, SerialError>;
}
