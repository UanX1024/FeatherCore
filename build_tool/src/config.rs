//! Configuration Module
//! 配置解析模块
//!
//! This module handles parsing of board configuration files (.toml).
//! 用于解析板级配置文件 (board.toml)

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Board configuration structure
/// 板级配置结构体
#[derive(Debug, Clone)]
pub struct BoardConfig {
    pub chip_name: String,
    pub chip_vendor: String,
    pub cpu_core: String,
    pub fpu_enabled: bool,
    pub clock_hz: u64,
    pub flash_base: u64,
    pub flash_size: u64,
    pub sram_base: u64,
    pub sram_size: u64,
    pub boot_base: u64,
    pub boot_size: u64,
    pub kernel_base: u64,
    pub kernel_size: u64,
    pub kernel_stack_size: u64,
    pub kernel_heap_size: u64,
}

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

pub fn find_board_config(board_name: &str, root: &str) -> Option<PathBuf> {
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
                            let config_path = board_path.join("config").join("board.toml");
                            if config_path.exists() {
                                if let Ok(content) = fs::read_to_string(&config_path) {
                                    if content.contains(&format!("name = \"{}\"", board_name.replace("-", " "))) 
                                        || board_path.file_name()
                                            .map(|n| n.to_string_lossy() == board_name)
                                            .unwrap_or(false) {
                                        return Some(config_path);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn parse_board_config(config_path: &PathBuf) -> Option<BoardConfig> {
    let content = fs::read_to_string(config_path).ok()?;
    let value: toml::Value = content.parse().ok()?;
    
    let chip_name = value.get("chip")
        .and_then(|v| v.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();
    
    let chip_vendor = value.get("chip")
        .and_then(|v| v.get("vendor"))
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
        .to_string();
    
    let cpu_core = value.get("cpu")
        .and_then(|v| v.get("core"))
        .and_then(|v| v.as_str())
        .unwrap_or("cortex-m4")
        .to_string();
    
    let fpu_enabled = value.get("cpu")
        .and_then(|v| v.get("fpu"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    
    let clock_hz = value.get("cpu")
        .and_then(|v| v.get("clock_hz"))
        .and_then(|v| v.as_integer())
        .unwrap_or(180000000) as u64;
    
    let flash_base = value.get("memory")
        .and_then(|v| v.get("flash_base"))
        .and_then(|v| v.as_integer())
        .unwrap_or(0x08000000) as u64;
    
    let flash_size = value.get("memory")
        .and_then(|v| v.get("flash_size"))
        .and_then(|v| v.as_integer())
        .unwrap_or(2048 * 1024) as u64;
    
    let sram_base = value.get("memory")
        .and_then(|v| v.get("sram_base"))
        .and_then(|v| v.as_integer())
        .unwrap_or(0x20000000) as u64;
    
    let sram_size = value.get("memory")
        .and_then(|v| v.get("sram_size"))
        .and_then(|v| v.as_integer())
        .unwrap_or(256 * 1024) as u64;
    
    let boot_base = value.get("bootloader")
        .and_then(|v| v.get("boot_base_address").or_else(|| v.get("code_base")))
        .and_then(|v| v.as_integer())
        .unwrap_or(flash_base as i64) as u64;
    
    let boot_size = value.get("bootloader")
        .and_then(|v| v.get("boot_size").or_else(|| v.get("code_size")))
        .and_then(|v| v.as_integer())
        .unwrap_or(64 * 1024) as u64;
    
    let kernel_base = value.get("kernel")
        .and_then(|v| v.get("kernel_base_address").or_else(|| v.get("code_base")))
        .and_then(|v| v.as_integer())
        .unwrap_or((flash_base + boot_size) as i64) as u64;
    
    let kernel_size = value.get("kernel")
        .and_then(|v| v.get("kernel_size").or_else(|| v.get("code_size")))
        .and_then(|v| v.as_integer())
        .unwrap_or((flash_size - boot_size) as i64) as u64;
    
    let kernel_stack_size = value.get("kernel")
        .and_then(|v| v.get("kernel_stack_size"))
        .and_then(|v| v.as_integer())
        .unwrap_or(16 * 1024) as u64;
    
    let kernel_heap_size = value.get("kernel")
        .and_then(|v| v.get("kernel_heap_size"))
        .and_then(|v| v.as_integer())
        .unwrap_or(64 * 1024) as u64;
    
    Some(BoardConfig {
        chip_name,
        chip_vendor,
        cpu_core,
        fpu_enabled,
        clock_hz,
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
                            let config_path = board_path.join("config").join("board.toml");
                            if config_path.exists() {
                                let board_name = board_path.file_name()
                                    .map(|n| n.to_string_lossy().to_string())
                                    .unwrap_or_default();
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
