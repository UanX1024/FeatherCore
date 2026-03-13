//! Clock Traits
//! 时钟接口定义

/// Clock driver trait
/// 时钟驱动接口
pub trait ClockDriver {
    /// Initialize the clock system
    /// 初始化时钟系统
    fn init(&self) -> Result<(), ClockError>;
    
    /// Get current system clock frequency in Hz
    /// 获取当前系统时钟频率（Hz）
    fn get_system_clock(&self) -> u32;
    
    /// Set system clock frequency
    /// 设置系统时钟频率
    fn set_system_clock(&self, freq: u32) -> Result<(), ClockError>;
    
    /// Get peripheral clock frequency
    /// 获取外设时钟频率
    fn get_peripheral_clock(&self, peripheral: Peripheral) -> u32;
}

/// Peripheral types for clock management
/// 时钟管理的外设类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Peripheral {
    /// GPIO peripheral
    Gpio,
    /// UART peripheral
    Uart,
    /// I2C peripheral
    I2c,
    /// SPI peripheral
    Spi,
    /// Timer peripheral
    Timer,
    /// ADC peripheral
    Adc,
}
