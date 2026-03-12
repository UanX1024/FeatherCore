//! Utils Module
//! 工具函数模块
//!
//! This module provides utility functions for the build tool.

/// Print a formatted message
/// 打印格式化消息
pub fn print_info(msg: &str) {
    println!("[INFO] {}", msg);
}

/// Print an error message
/// 打印错误消息
pub fn print_error(msg: &str) {
    eprintln!("[ERROR] {}", msg);
}

/// Print a success message
/// 打印成功消息
pub fn print_success(msg: &str) {
    println!("[SUCCESS] {}", msg);
}

/// Print a warning message
/// 打印警告消息
pub fn print_warning(msg: &str) {
    println!("[WARNING] {}", msg);
}

/// Format size in bytes to human readable format
/// 将字节大小格式化为人类可读格式
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if bytes >= GB {
        format!("{} GB", bytes / GB)
    } else if bytes >= MB {
        format!("{} MB", bytes / MB)
    } else if bytes >= KB {
        format!("{} KB", bytes / KB)
    } else {
        format!("{} bytes", bytes)
    }
}

/// Format address to hex string
/// 将地址格式化为十六进制字符串
pub fn format_address(addr: u64) -> String {
    format!("0x{:08X}", addr)
}
