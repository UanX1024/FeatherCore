//! Driver framework for FeatherCore
//! FeatherCore 驱动框架
//! 
//! This module provides a framework for device drivers in FeatherCore.
//! 此模块为 FeatherCore 提供设备驱动框架。

/// Driver trait for all device drivers
/// 所有设备驱动的 trait
pub trait Driver {
    /// Initialize the driver
    /// 初始化驱动
    fn init(&self) -> crate::Result<()>;
    
    /// Get the driver name
    /// 获取驱动名称
    fn name(&self) -> &str;
}

/// Driver manager for managing multiple drivers
/// 用于管理多个驱动的驱动管理器
pub struct DriverManager {
    drivers: [Option<&'static dyn Driver>; 16],
    driver_count: usize,
}

impl DriverManager {
    /// Create a new driver manager
    /// 创建一个新的驱动管理器
    pub const fn new() -> Self {
        Self {
            drivers: [None; 16],
            driver_count: 0,
        }
    }
    
    /// Add a driver to the manager
    /// 向管理器添加驱动
    pub fn add_driver(&mut self, driver: &'static dyn Driver) -> crate::Result<()> {
        if self.driver_count >= 16 {
            return Err(crate::Error::OutOfMemory);
        }
        
        self.drivers[self.driver_count] = Some(driver);
        self.driver_count += 1;
        Ok(())
    }
    
    /// Initialize all drivers
    /// 初始化所有驱动
    pub fn init_all(&self) -> crate::Result<()> {
        for i in 0..self.driver_count {
            if let Some(driver) = &self.drivers[i] {
                driver.init()?;
            }
        }
        Ok(())
    }
    
    /// Get the number of drivers
    /// 获取驱动数量
    pub fn driver_count(&self) -> usize {
        self.driver_count
    }
}

// Device tree aware drivers
// 设备树感知驱动
pub mod led;
pub mod gpio;
pub mod i2c;
pub mod serial;
pub mod spi;
pub mod i2s;
pub mod timer;