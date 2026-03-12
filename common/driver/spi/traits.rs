//! SPI Driver Traits
//! SPI 驱动特征定义
//!
//! This module defines the traits that all SPI drivers must implement.
//! It provides a common interface for SPI operations independent of
//! the underlying hardware platform.
//! 该模块定义了所有 SPI 驱动必须实现的特征。它提供了与底层硬件平台无关的
//! 通用 SPI 操作接口。

use crate::driver::spi::{SpiBuffer, SpiConfig, SpiError};

/// SPI driver state / SPI 驱动状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiState {
    /// Not ready / 未就绪
    NotReady,
    /// Ready / 就绪
    Ready,
    /// Busy / 忙碌
    Busy,
    /// Error / 错误
    Error,
}

/// SPI driver trait that must be implemented by all platform-specific drivers
/// SPI 驱动特征，所有平台特定的驱动都必须实现
pub trait SpiDriver {
    /// Initialize the SPI driver with the given configuration
    /// 使用给定配置初始化 SPI 驱动
    ///
    /// # Arguments / 参数
    /// * `config` - SPI configuration / SPI 配置
    ///
    /// # Returns / 返回
    /// * `Result<(), SpiError>` - Initialization result / 初始化结果
    fn init(&mut self, config: &SpiConfig) -> Result<(), SpiError>;

    /// Configure the SPI driver
    /// 配置 SPI 驱动
    ///
    /// # Arguments / 参数
    /// * `config` - SPI configuration / SPI 配置
    ///
    /// # Returns / 返回
    /// * `Result<(), SpiError>` - Configuration result / 配置结果
    fn configure(&mut self, config: &SpiConfig) -> Result<(), SpiError>;

    /// Full duplex transfer (transmit and receive simultaneously)
    /// 全双工传输 (同时发送和接收)
    ///
    /// # Arguments / 参数
    /// * `tx_buffer` - Data to transmit / 要发送的数据
    /// * `rx_buffer` - Buffer to receive data / 接收数据的缓冲区
    ///
    /// # Returns / 返回
    /// * `Result<usize, SpiError>` - Number of bytes transferred / 传输的字节数
    fn transfer(&mut self, tx_buffer: &[u8], rx_buffer: &mut [u8]) -> Result<usize, SpiError>;

    /// Write data to SPI bus
    /// 向 SPI 总线写入数据
    ///
    /// # Arguments / 参数
    /// * `data` - Data to write / 要写入的数据
    ///
    /// # Returns / 返回
    /// * `Result<usize, SpiError>` - Number of bytes written / 写入的字节数
    fn write(&mut self, data: &[u8]) -> Result<usize, SpiError>;

    /// Read data from SPI bus
    /// 从 SPI 总线读取数据
    ///
    /// # Arguments / 参数
    /// * `buffer` - Buffer to store read data / 存储读取数据的缓冲区
    ///
    /// # Returns / 返回
    /// * `Result<usize, SpiError>` - Number of bytes read / 读取的字节数
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, SpiError>;

    /// Set chip select
    /// 设置片选
    ///
    /// # Arguments / 参数
    /// * `cs` - Chip select number / 片选编号
    /// * `active` - Active state / 激活状态
    ///
    /// # Returns / 返回
    /// * `Result<(), SpiError>` - Result / 结果
    fn set_cs(&mut self, cs: u8, active: bool) -> Result<(), SpiError>;

    /// Get current SPI state
    /// 获取当前 SPI 状态
    ///
    /// # Returns / 返回
    /// * `SpiState` - Current state / 当前状态
    fn state(&self) -> SpiState;

    /// Check if SPI is ready
    /// 检查 SPI 是否就绪
    ///
    /// # Returns / 返回
    /// * `bool` - True if ready / 如果就绪返回 true
    fn is_ready(&self) -> bool {
        self.state() == SpiState::Ready
    }

    /// Enable the SPI peripheral
    /// 使能 SPI 外设
    fn enable(&mut self) -> Result<(), SpiError>;

    /// Disable the SPI peripheral
    /// 禁用 SPI 外设
    fn disable(&mut self) -> Result<(), SpiError>;

    /// Release SPI bus (release lock)
    /// 释放 SPI 总线 (释放锁)
    ///
    /// # Returns / 返回
    /// * `Result<(), SpiError>` - Result / 结果
    fn release(&mut self) -> Result<(), SpiError>;
}
