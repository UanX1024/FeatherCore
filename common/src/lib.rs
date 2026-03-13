//! FeatherCore Common Library
//! FeatherCore 公共库 (聚合库)
//! 
//! This is an aggregation library that re-exports sub-libraries.
//! 具体功能实现请参考各个子包。

#![no_std]

// Re-export architecture sub-libraries
#[cfg(all(feature = "arm", not(feature = "riscv")))]
pub use feathercore_arch_arm as arch;

#[cfg(all(feature = "riscv", not(feature = "arm")))]
pub use feathercore_arch_riscv as arch;

// Re-export driver sub-library
#[cfg(feature = "driver")]
pub use feathercore_driver as driver;

// Re-export generated sub-library
#[cfg(feature = "devicetree")]
pub use feathercore_generated as generated;

// Re-export platform module
pub mod platform;
