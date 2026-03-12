//! Configuration Module
//! 配置解析模块
//!
//! This module handles parsing of board and chip configuration files (.toml).
//! 用于解析板级和芯片级配置文件

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Board configuration structure
/// 板级配置结构体
#[derive(Debug, Clone)]
pub struct BoardConfig {
    /// Referenced chip name
    pub chip_name: String,
    /// Chip vendor
    pub chip_vendor: String,
    /// Board name
    pub board_name: String,
    /// Board vendor
    pub board_vendor: String,
    /// CPU core type (e.g., "cortex-m4")
    pub cpu_core: String,
    /// CPU frequency in Hz
    pub clock_hz: u64,
    /// Whether FPU is enabled
    pub fpu_enabled: bool,
    /// Flash base address
    pub flash_base: u64,
    /// Flash size in bytes
    pub flash_size: u64,
    /// SRAM base address
    pub sram_base: u64,
    /// SRAM size in bytes
    pub sram_size: u64,
    /// Bootloader base address
    pub boot_base: u64,
    /// Bootloader size in bytes
    pub boot_size: u64,
    /// Kernel base address
    pub kernel_base: u64,
    /// Kernel size in bytes
    pub kernel_size: u64,
    /// Kernel stack size in bytes
    pub kernel_stack_size: u64,
    /// Kernel heap size in bytes
    pub kernel_heap_size: u64,
}

/// Get target architecture based on CPU core
/// 根据 CPU 核心类型获取目标架构
pub fn get_target_arch(cpu_core: &str) -> String {
    match cpu_core {
        "cortex-m0" | "cortex-m0plus" => "thumbv6m-none-eabi".to_string(),
        "cortex-m3" => "thumbv7m-none-eabi".to_string(),
        "cortex-m4" | "cortex-m7" => "thumbv7em-none-eabihf".to_string(),
        "cortex-m23" => "thumbv8m.base-none-eabi".to_string(),
        "cortex-m33" | "cortex-m35p" | "cortex-m55" => "thumbv8m.main-none-eabihf".to_string(),
        "cortex-a5" | "cortex-a7" | "cortex-a8" | "cortex-a9" => "armv7a-none-eabihf".to_string(),
        "cortex-a53" | "cortex-a55" | "cortex-a72" | "cortex-a76" => "aarch64-none-elf".to_string(),
        "riscv32" | "riscv32imc" | "riscv32imac" => "riscv32imc-unknown-none-elf".to_string(),
        "riscv64" | "riscv64gc" => "riscv64gc-unknown-none-elf".to_string(),
        _ => "thumbv7em-none-eabihf".to_string(),
    }
}

/// Get architecture features based on CPU core
/// 根据 CPU 核心类型获取架构特性
pub fn get_arch_features(cpu_core: &str) -> Vec<String> {
    let mut features = Vec::new();
    match cpu_core {
        "cortex-m0" | "cortex-m0plus" => features.push("armv6-m".to_string()),
        "cortex-m3" => features.push("armv7-m".to_string()),
        "cortex-m4" | "cortex-m7" => features.push("armv7-em".to_string()),
        "cortex-m23" => features.push("armv8-m-base".to_string()),
        "cortex-m33" | "cortex-m35p" | "cortex-m55" => features.push("armv8-m-main".to_string()),
        "cortex-a5" | "cortex-a7" | "cortex-a8" | "cortex-a9" => features.push("armv7-a".to_string()),
        "cortex-a53" | "cortex-a55" | "cortex-a72" | "cortex-a76" => features.push("armv8-a".to_string()),
        "riscv32" | "riscv32imc" | "riscv32imac" => features.push("riscv".to_string()),
        "riscv64" | "riscv64gc" => features.push("riscv".to_string()),
        _ => features.push("armv7-em".to_string()),
    }
    features
}

/// Find board configuration file
/// 查找板级配置文件
///
/// New structure: platform/board/<vendor>/<board_name>/<board_name>_defconfig.toml
pub fn find_board_config(board_name: &str, root: &str) -> Option<PathBuf> {
    let root_path = PathBuf::from(root);
    let platform_path = root_path.join("platform").join("board");
    
    if let Ok(entries) = fs::read_dir(&platform_path) {
        for vendor_entry in entries.flatten() {
            let vendor_path = vendor_entry.path();
            if vendor_path.is_dir() {
                let board_path = vendor_path.join(board_name);
                if board_path.is_dir() {
                    let config_path = board_path.join(format!("{}_defconfig.toml", board_name));
                    if config_path.exists() {
                        return Some(config_path);
                    }
                }
            }
        }
    }
    None
}

/// Find chip configuration file
/// 查找芯片配置文件
///
/// New structure: platform/chip/<vendor>/<chip_name>/<chip_name>_defconfig.toml
pub fn find_chip_config(chip_name: &str, root: &str) -> Option<PathBuf> {
    let root_path = PathBuf::from(root);
    let platform_path = root_path.join("platform").join("chip");
    
    if let Ok(entries) = fs::read_dir(&platform_path) {
        for vendor_entry in entries.flatten() {
            let vendor_path = vendor_entry.path();
            if vendor_path.is_dir() {
                let chip_path = vendor_path.join(chip_name);
                if chip_path.is_dir() {
                    let config_path = chip_path.join(format!("{}_defconfig.toml", chip_name));
                    if config_path.exists() {
                        return Some(config_path);
                    }
                }
            }
        }
    }
    None
}

/// Merge chip and board configurations
/// 合并芯片和板级配置
fn merge_configs(chip_config: &toml::Value, board_config: &toml::Value) -> toml::Value {
    let mut merged = chip_config.clone();
    
    // Merge [board] section if exists
    if let Some(board_section) = board_config.get("board") {
        if let Some(merged_board) = merged.get_mut("board") {
            if let Some(merged_board_map) = merged_board.as_table_mut() {
                if let Some(board_map) = board_section.as_table() {
                    for (k, v) in board_map {
                        merged_board_map.insert(k.clone(), v.clone());
                    }
                }
            }
        }
    }
    
    // Board-level [cpu], [memory], [bootloader], [kernel] override chip settings
    for section in &["cpu", "memory", "bootloader", "kernel"] {
        if let Some(board_section) = board_config.get(*section) {
            if let Some(merged_section) = merged.get_mut(*section) {
                if let Some(merged_map) = merged_section.as_table_mut() {
                    if let Some(board_map) = board_section.as_table() {
                        for (k, v) in board_map {
                            merged_map.insert(k.clone(), v.clone());
                        }
                    }
                }
            }
        }
    }
    
    merged
}

/// Parse board configuration file and merge with chip config
/// 解析板级配置文件并与芯片配置合并
pub fn parse_board_config(config_path: &PathBuf) -> Option<BoardConfig> {
    let root = super::root_path::get_root_path();
    let content = fs::read_to_string(config_path).ok()?;
    let board_value: toml::Value = content.parse().ok()?;
    
    // Get chip reference
    let chip_name = board_value.get("chip")
        .and_then(|v| v.as_str())
        .unwrap_or("stm32f4")
        .to_string();
    
    // Find and parse chip config
    let chip_path = match find_chip_config(&chip_name, &root) {
        Some(p) => p,
        None => {
            eprintln!("Warning: Chip config '{}' not found, using board config only", chip_name);
            return parse_board_config_only(&board_value);
        }
    };
    
    let chip_content = fs::read_to_string(&chip_path).ok()?;
    let chip_value: toml::Value = chip_content.parse().ok()?;
    
    // Merge configs (chip as base, board overrides)
    let merged = merge_configs(&chip_value, &board_value);
    
    parse_merged_config(&merged, &board_value)
}

/// Parse board config without chip merge (fallback)
fn parse_board_config_only(board_value: &toml::Value) -> Option<BoardConfig> {
    parse_merged_config(board_value, board_value)
}

/// Parse merged configuration
fn parse_merged_config(merged: &toml::Value, board_value: &toml::Value) -> Option<BoardConfig> {
    // Get chip info from [chip] section
    let chip_name = merged.get("chip")
        .and_then(|v| v.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();
    
    let chip_vendor = merged.get("chip")
        .and_then(|v| v.get("vendor"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();
    
    // Get board info from [board] section
    let board_name = board_value.get("board")
        .and_then(|v| v.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();
    
    let board_vendor = board_value.get("board")
        .and_then(|v| v.get("vendor"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();
    
    let cpu_core = merged.get("cpu")
        .and_then(|v| v.get("core"))
        .and_then(|v| v.as_str())
        .unwrap_or("cortex-m4")
        .to_string();
    
    let clock_hz = merged.get("cpu")
        .and_then(|v| v.get("frequency").or_else(|| v.get("clock_hz")))
        .and_then(|v| v.as_integer())
        .unwrap_or(180000000) as u64;
    
    let fpu_enabled = merged.get("cpu")
        .and_then(|v| v.get("fpu"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    
    let flash_base = merged.get("memory")
        .and_then(|v| v.get("flash_base"))
        .and_then(|v| v.as_integer())
        .unwrap_or(0x08000000) as u64;
    
    let flash_size = merged.get("memory")
        .and_then(|v| v.get("flash_size"))
        .and_then(|v| v.as_integer())
        .unwrap_or(2048 * 1024) as u64;
    
    let sram_base = merged.get("memory")
        .and_then(|v| v.get("sram_base"))
        .and_then(|v| v.as_integer())
        .unwrap_or(0x20000000) as u64;
    
    let sram_size = merged.get("memory")
        .and_then(|v| v.get("sram_size"))
        .and_then(|v| v.as_integer())
        .unwrap_or(256 * 1024) as u64;
    
    let boot_base = merged.get("bootloader")
        .and_then(|v| v.get("boot_base_address"))
        .and_then(|v| v.as_integer())
        .unwrap_or(flash_base as i64) as u64;
    
    let boot_size = merged.get("bootloader")
        .and_then(|v| v.get("boot_size"))
        .and_then(|v| v.as_integer())
        .unwrap_or(64 * 1024) as u64;
    
    let kernel_base = merged.get("kernel")
        .and_then(|v| v.get("kernel_base_address"))
        .and_then(|v| v.as_integer())
        .unwrap_or((flash_base + boot_size) as i64) as u64;
    
    let kernel_size = merged.get("kernel")
        .and_then(|v| v.get("kernel_size"))
        .and_then(|v| v.as_integer())
        .unwrap_or((flash_size - boot_size) as i64) as u64;
    
    let kernel_stack_size = merged.get("kernel")
        .and_then(|v| v.get("kernel_stack_size"))
        .and_then(|v| v.as_integer())
        .unwrap_or(16 * 1024) as u64;
    
    let kernel_heap_size = merged.get("kernel")
        .and_then(|v| v.get("kernel_heap_size"))
        .and_then(|v| v.as_integer())
        .unwrap_or(64 * 1024) as u64;
    
    Some(BoardConfig {
        chip_name,
        chip_vendor,
        board_name,
        board_vendor,
        cpu_core,
        clock_hz,
        fpu_enabled,
        flash_base,
        flash_size,
        sram_base,
        sram_size,
        boot_base,
        boot_size,
        kernel_base,
        kernel_size,
        kernel_stack_size,
        kernel_heap_size,
    })
}

/// List all available boards
/// 列出所有可用的开发板
pub fn list_available_boards(root: &str) -> HashMap<String, PathBuf> {
    let mut boards = HashMap::new();
    let root_path = PathBuf::from(root);
    let platform_path = root_path.join("platform").join("board");
    
    if let Ok(entries) = fs::read_dir(&platform_path) {
        for vendor_entry in entries.flatten() {
            let vendor_path = vendor_entry.path();
            if vendor_path.is_dir() {
                if let Ok(sub_entries) = fs::read_dir(&vendor_path) {
                    for board_entry in sub_entries.flatten() {
                        let board_path = board_entry.path();
                        if board_path.is_dir() {
                            let board_name = board_path.file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_default();
                            let config_path = board_path.join(format!("{}_defconfig.toml", board_name));
                            if config_path.exists() {
                                boards.insert(board_name, config_path);
                            }
                        }
                    }
                }
            }
        }
    }
    
    boards
}
