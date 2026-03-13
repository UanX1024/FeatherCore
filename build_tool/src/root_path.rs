//! Root Path Module
//! 根路径管理模块
//!
//! This module manages the root path of FeatherCore project.
//! The root path is used to locate platform configuration files,
//! output linker scripts, and invoke cargo build commands.

use std::sync::OnceLock;
use std::env;

/// Global root path storage using OnceLock for thread-safety
static ROOT_PATH: OnceLock<String> = OnceLock::new();

/// Get the root path
/// If not set, returns the current working directory
/// 获取根路径
pub fn get_root_path() -> String {
    ROOT_PATH.get_or_init(|| {
        env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "../../..".to_string())
    }).clone()
}

/// Set the root path
pub fn set_root_path(path: &str) {
    let _ = ROOT_PATH.set(path.to_string());
}
