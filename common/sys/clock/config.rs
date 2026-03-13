//! Clock Configuration
//! 时钟配置

/// Clock configuration structure
/// 时钟配置结构
#[derive(Debug, Clone, Copy, Default)]
pub struct ClockConfig {
    /// System clock frequency in Hz
    /// 系统时钟频率（Hz）
    pub system_clock: u32,
    /// Peripheral clock frequencies in Hz
    /// 外设时钟频率（Hz）
    pub peripheral_clocks: PeripheralClocks,
    /// Clock source configuration
    /// 时钟源配置
    pub clock_source: ClockSource,
}

/// Peripheral clock frequencies
/// 外设时钟频率
#[derive(Debug, Clone, Copy, Default)]
pub struct PeripheralClocks {
    /// GPIO clock frequency
    pub gpio: u32,
    /// UART clock frequency
    pub uart: u32,
    /// I2C clock frequency
    pub i2c: u32,
    /// SPI clock frequency
    pub spi: u32,
    /// Timer clock frequency
    pub timer: u32,
    /// ADC clock frequency
    pub adc: u32,
}

/// Clock source configuration
/// 时钟源配置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockSource {
    /// Internal RC oscillator
    InternalRc,
    /// External crystal oscillator
    ExternalCrystal,
    /// External clock input
    ExternalClock,
    /// PLL (Phase-Locked Loop)
    Pll,
}
