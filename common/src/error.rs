//! Error handling utilities
//! 错误处理工具
//! 
//! This module provides utilities for error handling in FeatherCore.
//! 此模块为FeatherCore中的错误处理提供实用工具。

use crate::{Error, Result};

/// Extension trait for Result types
/// Result类型的扩展特性
/// 
/// Provides a method to map any error to FeatherCore's Error type.
/// 提供将任何错误映射到FeatherCore的Error类型的方法。
pub trait ResultExt<T> {
    /// Map an error to FeatherCore Error
    /// 将错误映射到FeatherCore Error
    fn feather_err(self) -> Result<T>;
}

impl<T, E> ResultExt<T> for core::result::Result<T, E> {
    fn feather_err(self) -> Result<T> {
        self.map_err(|_| Error::Other)
    }
}

/// Check if a condition is true, otherwise return an error
/// 检查条件是否为真，否则返回错误
/// 
/// # Example
/// ```
/// use feathercore_common::{ensure, Error};
/// 
/// fn example(value: u32) -> Result<(), Error> {
///     ensure!(value > 0, Error::InvalidArgument);
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}

/// Return early with an error if a condition is not met
/// 如果条件不满足，提前返回错误
/// 
/// # Example
/// ```
/// use feathercore_common::{require, Error};
/// 
/// fn example(value: u32) -> Result<(), Error> {
///     require!(value > 0, Error::InvalidArgument);
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! require {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}