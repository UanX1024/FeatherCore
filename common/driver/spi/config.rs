//! SPI Driver Configuration
//! SPI 驱动配置
//!
//! This module defines the configuration structures for SPI driver.
//! 该模块定义了 SPI 驱动的配置结构。

/// SPI clock polarity (CPOL) / SPI 时钟极性
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum SpiClockPolarity {
    /// Clock low when idle / 空闲时时钟低电平
    #[default]
    LowWhenIdle = 0,
    /// Clock high when idle / 空闲时时钟高电平
    HighWhenIdle = 1,
}

/// SPI clock phase (CPHA) / SPI 时钟相位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum SpiClockPhase {
    /// Sample on first clock edge / 第一个时钟沿采样
    #[default]
    FirstEdge = 0,
    /// Sample on second clock edge / 第二个时钟沿采样
    SecondEdge = 1,
}

/// SPI operation mode / SPI 工作模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum SpiMode {
    /// Master mode / 主模式
    #[default]
    Master,
    /// Slave mode / 从模式
    Slave,
}

/// SPI data bit order / SPI 数据位顺序
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum SpiBitOrder {
    /// Most significant bit first / 高位先传输
    #[default]
    MsbFirst,
    /// Least significant bit first / 低位先传输
    LsbFirst,
}

/// SPI data lines / SPI 数据线数量
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum SpiDataLines {
    /// Single line (full duplex) / 单线 (全双工)
    #[default]
    Single,
    /// Dual line / 双线
    Dual,
    /// Quad line / 四线
    Quad,
    /// Octal line / 八线
    Octal,
}

/// SPI operation flags / SPI 操作标志
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SpiOperation(pub u32);

impl SpiOperation {
    /// Master mode / 主模式
    pub const MASTER: Self = Self(1 << 0);
    /// Slave mode / 从模式
    pub const SLAVE: Self = Self(1 << 1);
    /// Transfer MSB first / 高位先传输
    pub const TRANSFER_MSB: Self = Self(1 << 2);
    /// Transfer LSB first / 低位先传输
    pub const TRANSFER_LSB: Self = Self(1 << 3);
    /// Hold CS active after transfer / 传输后保持片选
    pub const HOLD_ON_CS: Self = Self(1 << 4);
    /// Loopback mode / 回环模式
    pub const LOOP: Self = Self(1 << 5);
    /// Chip select active high / 片选高电平有效
    pub const CS_ACTIVE_HIGH: Self = Self(1 << 6);
}

/// SPI configuration structure / SPI 配置结构体
#[derive(Debug, Clone, Default)]
pub struct SpiConfig {
    /// SPI frequency in Hz / SPI 频率 (Hz)
    pub frequency: u32,
    /// Clock polarity / 时钟极性
    pub polarity: SpiClockPolarity,
    /// Clock phase / 时钟相位
    pub phase: SpiClockPhase,
    /// Operation mode (master/slave) / 工作模式 (主/从)
    pub mode: SpiMode,
    /// Bit order / 位顺序
    pub bit_order: SpiBitOrder,
    /// Data lines / 数据线数量
    pub data_lines: SpiDataLines,
    /// Word size in bits / 字长 (位)
    pub word_size: u8,
    /// Chip select / 片选
    pub chip_select: u8,
    /// Enable DMA / 使能 DMA
    pub dma_enabled: bool,
    /// DMA buffer size / DMA 缓冲区大小
    pub dma_buffer_size: usize,
}

impl SpiConfig {
    /// Create a new default SPI configuration / 创建默认 SPI 配置
    pub fn new() -> Self {
        Self::default()
    }

    /// Create master mode configuration / 创建主模式配置
    pub fn master() -> Self {
        let mut config = Self::default();
        config.mode = SpiMode::Master;
        config
    }

    /// Create slave mode configuration / 创建从模式配置
    pub fn slave() -> Self {
        let mut config = Self::default();
        config.mode = SpiMode::Slave;
        config
    }

    /// Set frequency / 设置频率
    pub fn with_frequency(mut self, freq: u32) -> Self {
        self.frequency = freq;
        self
    }

    /// Set clock polarity / 设置时钟极性
    pub fn with_polarity(mut self, polarity: SpiClockPolarity) -> Self {
        self.polarity = polarity;
        self
    }

    /// Set clock phase / 设置时钟相位
    pub fn with_phase(mut self, phase: SpiClockPhase) -> Self {
        self.phase = phase;
        self
    }

    /// Set bit order / 设置位顺序
    pub fn with_bit_order(mut self, order: SpiBitOrder) -> Self {
        self.bit_order = order;
        self
    }

    /// Set data lines / 设置数据线
    pub fn with_data_lines(mut self, lines: SpiDataLines) -> Self {
        self.data_lines = lines;
        self
    }

    /// Set word size / 设置字长
    pub fn with_word_size(mut self, size: u8) -> Self {
        self.word_size = size;
        self
    }

    /// Set chip select / 设置片选
    pub fn with_chip_select(mut self, cs: u8) -> Self {
        self.chip_select = cs;
        self
    }

    /// Enable DMA / 使能 DMA
    pub fn with_dma(mut self, enabled: bool, buffer_size: usize) -> Self {
        self.dma_enabled = enabled;
        self.dma_buffer_size = buffer_size;
        self
    }
}

/// SPI buffer structure / SPI 缓冲区结构
#[derive(Debug, Clone)]
pub struct SpiBuffer {
    /// Buffer pointer / 缓冲区指针
    pub ptr: *mut u8,
    /// Buffer length / 缓冲区长度
    pub len: usize,
}

impl SpiBuffer {
    /// Create buffer from slice / 从切片创建缓冲区
    pub fn from_slice(slice: &mut [u8]) -> Self {
        Self {
            ptr: slice.as_mut_ptr(),
            len: slice.len(),
        }
    }
}

/// SPI buffer set / SPI 缓冲区集
#[derive(Debug, Clone)]
pub struct SpiBufferSet {
    /// Buffers / 缓冲区数组
    pub buffers: &'static mut [SpiBuffer],
    /// Number of buffers / 缓冲区数量
    pub count: usize,
}
