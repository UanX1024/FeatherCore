//! Driver framework for FeatherCore
//! 
//! This module provides a framework for device drivers in FeatherCore.

/// Driver trait for all device drivers
pub trait Driver {
    /// Initialize the driver
    fn init(&self) -> crate::Result<()>;
    
    /// Get the driver name
    fn name(&self) -> &str;
}

/// Driver manager for managing multiple drivers
pub struct DriverManager {
    drivers: [Option<&'static dyn Driver>; 16],
    driver_count: usize,
}

impl DriverManager {
    /// Create a new driver manager
    pub const fn new() -> Self {
        Self {
            drivers: [None; 16],
            driver_count: 0,
        }
    }
    
    /// Add a driver to the manager
    pub fn add_driver(&mut self, driver: &'static dyn Driver) -> crate::Result<()> {
        if self.driver_count >= 16 {
            return Err(crate::Error::OutOfMemory);
        }
        
        self.drivers[self.driver_count] = Some(driver);
        self.driver_count += 1;
        Ok(())
    }
    
    /// Initialize all drivers
    pub fn init_all(&self) -> crate::Result<()> {
        for i in 0..self.driver_count {
            if let Some(driver) = &self.drivers[i] {
                driver.init()?;
            }
        }
        Ok(())
    }
    
    /// Get the number of drivers
    pub fn driver_count(&self) -> usize {
        self.driver_count
    }
}