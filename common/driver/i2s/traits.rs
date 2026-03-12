//! I2S Driver Traits
//! I2S 驱动特征定义
//!
//! This module defines the traits that all I2S drivers must implement.
//! It provides a common interface for I2S operations independent of
//! the underlying hardware platform.
//! 该模块定义了所有 I2S 驱动必须实现的特征。它提供了与底层硬件平台无关的
//! 通用 I2S 操作接口。

use crate::driver::i2s::{I2sConfig, I2sDir, I2sError};

/// I2S driver state / I2S 驱动状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2sState {
    /// Not ready / 未就绪
    NotReady,
    /// Ready / 就绪
    Ready,
    /// Running / 运行中
    Running,
    /// Stopping / 停止中
    Stopping,
    /// Error / 错误
    Error,
}

/// I2S driver trait that must be implemented by all platform-specific drivers
/// I2S 驱动特征，所有平台特定的驱动都必须实现
pub trait I2sDriver {
    /// Initialize the I2S driver with the given configuration
    /// 使用给定配置初始化 I2S 驱动
    ///
    /// # Arguments / 参数
    /// * `config` - I2S configuration / I2S 配置
    ///
    /// # Returns / 返回
    /// * `Result<(), I2sError>` - Initialization result / 初始化结果
    fn init(&mut self, config: &I2sConfig) -> Result<(), I2sError>;

    /// Configure the I2S driver for specific direction
    /// 配置 I2S 驱动的特定方向
    ///
    /// # Arguments / 参数
    /// * `dir` - Direction (Tx/Rx/Both) / 方向 (发送/接收/双向)
    /// * `config` - I2S configuration / I2S 配置
    ///
    /// # Returns / 返回
    /// * `Result<(), I2sError>` - Configuration result / 配置结果
    fn configure(&mut self, dir: I2sDir, config: &I2sConfig) -> Result<(), I2sError>;

    /// Write data to I2S TX buffer
    /// 向 I2S 发送缓冲区写入数据
    ///
    /// # Arguments / 参数
    /// * `data` - Data buffer to write / 要写入的数据缓冲区
    ///
    /// # Returns / 返回
    /// * `Result<usize, I2sError>` - Number of bytes written / 写入的字节数
    fn write(&mut self, data: &[u8]) -> Result<usize, I2sError>;

    /// Read data from I2S RX buffer
    /// 从 I2S 接收缓冲区读取数据
    ///
    /// # Arguments / 参数
    /// * `buffer` - Buffer to store read data / 存储读取数据的缓冲区
    ///
    /// # Returns / 返回
    /// * `Result<usize, I2sError>` - Number of bytes read / 读取的字节数
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, I2sError>;

    /// Start I2S transfer
    /// 启动 I2S 传输
    ///
    /// # Arguments / 参数
    /// * `dir` - Direction to start / 要启动的方向
    ///
    /// # Returns / 返回
    /// * `Result<(), I2sError>` - Start result / 启动结果
    fn start(&mut self, dir: I2sDir) -> Result<(), I2sError>;

    /// Stop I2S transfer
    /// 停止 I2S 传输
    ///
    /// # Arguments / 参数
    /// * `dir` - Direction to stop / 要停止的方向
    ///
    /// # Returns / 返回
    /// * `Result<(), I2sError>` - Stop result / 停止结果
    fn stop(&mut self, dir: I2sDir) -> Result<(), I2sError>;

    /// Get current I2S state
    /// 获取当前 I2S 状态
    ///
    /// # Returns / 返回
    /// * `I2sState` - Current state / 当前状态
    fn state(&self) -> I2sState;

    /// Check if I2S is ready
    /// 检查 I2S 是否就绪
    ///
    /// # Returns / 返回
    /// * `bool` - True if ready / 如果就绪返回 true
    fn is_ready(&self) -> bool {
        self.state() == I2sState::Ready
    }

    /// Enable the I2S peripheral
    /// 使能 I2S 外设
    fn enable(&mut self) -> Result<(), I2sError>;

    /// Disable the I2S peripheral
    /// 禁用 I2S 外设
    fn disable(&mut self) -> Result<(), I2sError>;

    /// Get the TX buffer available size
    /// 获取发送缓冲区可用大小
    ///
    /// # Returns / 返回
    /// * `Result<usize, I2sError>` - Available size / 可用大小
    fn tx_available(&self) -> Result<usize, I2sError>;

    /// Get the RX buffer available size
    /// 获取接收缓冲区可用大小
    ///
    /// # Returns / 返回
    /// * `Result<usize, I2sError>` - Available size / 可用大小
    fn rx_available(&self) -> Result<usize, I2sError>;
}
