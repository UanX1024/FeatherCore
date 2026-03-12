//! Root Path Module
//! 根路径管理模块
//!
//! This module manages the root path of FeatherCore project.
//! The root path is used to locate platform configuration files,
//! output linker scripts, and invoke cargo build commands.

use std::sync::OnceLock;
use std::env;

/// Global root path storage using OnceLock for thread-safety
/// 使用 OnceLock 实现线程安全的全局根路径存储
static ROOT_PATH: OnceLock<String> = OnceLock::new();

/// Get the root path
/// If not set, returns the default path relative to build_tool directory
/// 获取根路径
/// 如果未设置，返回相对于 build_tool 目录的默认路径
///
/// # Returns
/// The root path as a String
/// 根路径字符串
pub fn get_root_path() -> String {
    ROOT_PATH.get_or_init(|| {
        env::current_dir()
            .unwrap_or_default()
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "../../..".to_string())
    }).clone()
}

/// Set the root path
/// 设置根路径
///
/// # Arguments
/// * `path` - The absolute path to FeatherCore root directory
///             FeatherCore 根目录的绝对路径
pub fn set_root_path(path: &str) {
    let _ = ROOT_PATH.set(path.to_string());
}
