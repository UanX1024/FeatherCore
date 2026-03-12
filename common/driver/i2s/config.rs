//! I2S Driver Configuration
//! I2S 驱动配置
//!
//! This module defines the configuration structures for I2S driver.
//! 该模块定义了 I2S 驱动的配置结构。

/// I2S data format / I2S 数据格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum I2sFormat {
    /// Standard I2S format / 标准 I2S 格式
    #[default]
    I2s = 0,
    /// PCM short frame sync / PCM 短帧同步
    PcmShort,
    /// PCM long frame sync / PCM 长帧同步
    PcmLong,
    /// Left justified / 左对齐
    LeftJustified,
    /// Right justified / 右对齐
    RightJustified,
}

/// I2S operation mode / I2S 工作模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum I2sMode {
    /// Master mode / 主模式
    #[default]
    Master,
    /// Slave mode / 从模式
    Slave,
}

/// I2S direction / I2S 传输方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum I2sDir {
    /// Transmit mode / 发送模式
    #[default]
    Tx,
    /// Receive mode / 接收模式
    Rx,
    /// Bidirectional mode / 双向模式
    Both,
}

/// I2S word size / I2S 字长
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum I2sWordSize {
    /// 8 bits / 8 位
    #[default]
    Bits8 = 8,
    /// 10 bits / 10 位
    Bits10 = 10,
    /// 12 bits / 12 位
    Bits12 = 12,
    /// 16 bits / 16 位
    Bits16 = 16,
    /// 20 bits / 20 位
    Bits20 = 20,
    /// 24 bits / 24 位
    Bits24 = 24,
    /// 32 bits / 32 位
    Bits32 = 32,
}

/// I2S clock polarity / I2S 时钟极性
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum I2sClockPolarity {
    /// Clock low when idle / 空闲时时钟低电平
    #[default]
    LowWhenIdle,
    /// Clock high when idle / 空闲时时钟高电平
    HighWhenIdle,
}

/// I2S frame synchronization / I2S 帧同步
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum I2sFrameSync {
    /// Frame sync pulse for each word / 每个字产生一个帧同步脉冲
    #[default]
    PerWord,
    /// Frame sync pulse for each frame / 每帧产生一个帧同步脉冲
    PerFrame,
}

/// I2S configuration structure / I2S 配置结构体
#[derive(Debug, Clone, Default)]
pub struct I2sConfig {
    /// I2S format / I2S 格式
    pub format: I2sFormat,
    /// I2S mode (master/slave) / I2S 模式 (主/从)
    pub mode: I2sMode,
    /// Sample rate in Hz / 采样率 (Hz)
    pub sample_rate: u32,
    /// Word size / 字长
    pub word_size: I2sWordSize,
    /// Number of channels / 声道数
    pub channels: u8,
    /// Clock polarity / 时钟极性
    pub clock_polarity: I2sClockPolarity,
    /// Frame sync mode / 帧同步模式
    pub frame_sync: I2sFrameSync,
    /// Bit clock continuous or gated / 位时钟连续或门控
    pub bit_clock_gated: bool,
    /// Frame clock continuous or gated / 帧时钟连续或门控
    pub frame_clock_gated: bool,
    /// Data justification / 数据对齐方式
    pub data_justification: bool,
    /// Enable DMA / 使能 DMA
    pub dma_enabled: bool,
    /// DMA buffer size / DMA 缓冲区大小
    pub dma_buffer_size: usize,
}

impl I2sConfig {
    /// Create a new default I2S configuration / 创建默认 I2S 配置
    pub fn new() -> Self {
        Self::default()
    }

    /// Create configuration for TX mode / 创建发送模式配置
    pub fn tx() -> Self {
        let mut config = Self::default();
        config
    }

    /// Create configuration for RX mode / 创建接收模式配置
    pub fn rx() -> Self {
        let mut config = Self::default();
        config
    }

    /// Set sample rate / 设置采样率
    pub fn with_sample_rate(mut self, rate: u32) -> Self {
        self.sample_rate = rate;
        self
    }

    /// Set word size / 设置字长
    pub fn with_word_size(mut self, size: I2sWordSize) -> Self {
        self.word_size = size;
        self
    }

    /// Set number of channels / 设置声道数
    pub fn with_channels(mut self, channels: u8) -> Self {
        self.channels = channels;
        self
    }

    /// Set format / 设置格式
    pub fn with_format(mut self, format: I2sFormat) -> Self {
        self.format = format;
        self
    }

    /// Set mode (master/slave) / 设置模式 (主/从)
    pub fn with_mode(mut self, mode: I2sMode) -> Self {
        self.mode = mode;
        self
    }

    /// Enable DMA / 使能 DMA
    pub fn with_dma(mut self, enabled: bool, buffer_size: usize) -> Self {
        self.dma_enabled = enabled;
        self.dma_buffer_size = buffer_size;
        self
    }
}
