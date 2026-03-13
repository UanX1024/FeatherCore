//! Build Module
//! 构建功能模块
//!
//! This module contains the core build logic for the build tool.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::config;
use crate::device_tree;
use crate::linker;
use crate::root_path;
use crate::utils;

/// List supported boards
/// 列出支持的开发板
pub fn list_boards() {
    let root = root_path::get_root_path();
    let boards = config::list_available_boards(&root);
    
    println!("Supported boards:");
    println!("=================");
    
    if boards.is_empty() {
        println!("No boards found in platform/board/");
        return;
    }
    
    for (name, path) in &boards {
        println!("  - {} (config: {})", name, path.display());
    }
    
    println!();
    println!("Total: {} boards", boards.len());
}

/// Show board information
/// 显示开发板信息
pub fn show_board_info(board_name: &str) {
    let root = root_path::get_root_path();
    
    if let Some(config_path) = config::find_board_config(board_name, &root) {
        if let Some(board_config) = config::parse_board_config(&config_path) {
            println!("Board: {}", board_name);
            println!("====================");
            println!("Chip: {} ({})", board_config.chip_name, board_config.chip_vendor);
            println!("CPU: {} (FPU: {})", board_config.cpu_core, if board_config.fpu_enabled { "enabled" } else { "disabled" });
            println!("Clock: {} Hz", board_config.clock_hz);
            println!("Flash: {} KB @ 0x{:08X}", board_config.flash_size / 1024, board_config.flash_base);
            println!("RAM: {} KB @ 0x{:08X}", board_config.sram_size / 1024, board_config.sram_base);
            println!("Boot: {} KB @ 0x{:08X}", board_config.boot_size / 1024, board_config.boot_base);
            println!("Kernel: {} KB @ 0x{:08X}", board_config.kernel_size / 1024, board_config.kernel_base);
            println!("Target: {}", config::get_target_arch(&board_config.cpu_core));
            return;
        }
    }
    
    utils::print_error(&format!("Board '{}' not found", board_name));
}

/// Generate configuration for a board
/// 为开发板生成配置
pub fn generate_config(board_name: &str) {
    let root = root_path::get_root_path();
    
    println!("Generating configuration for board: {}", board_name);
    println!("Root path: {}", root);
    
    // Find board config
    let config_path = match config::find_board_config(board_name, &root) {
        Some(path) => path,
        None => {
            utils::print_error(&format!("Board '{}' not found", board_name));
            return;
        }
    };
    
    println!("Config file: {}", config_path.display());
    
    // Parse config
    let board_config = match config::parse_board_config(&config_path) {
        Some(cfg) => cfg,
        None => {
            utils::print_error("Failed to parse board configuration");
            return;
        }
    };
    
    // Ensure output directories exist
    let root_path = PathBuf::from(&root);
    fs::create_dir_all(root_path.join("boot")).unwrap_or_default();
    fs::create_dir_all(root_path.join("kernel")).unwrap_or_default();
    fs::create_dir_all(root_path.join("common/generated/src")).unwrap_or_default();
    
    // Generate linker scripts
    let boot_link_path = root_path.join("boot/link.x");
    let kernel_link_path = root_path.join("kernel/link.x");
    let common_dt_path = root_path.join("common/generated/src/devicetree.rs");
    
    let boot_script = linker::generate_boot_linker_script(&board_config);
    let kernel_script = linker::generate_kernel_linker_script(&board_config);
    
    fs::write(&boot_link_path, &boot_script).unwrap_or_default();
    fs::write(&kernel_link_path, &kernel_script).unwrap_or_default();
    
    println!("Generated linker scripts:");
    println!("  - {}", boot_link_path.display());
    println!("  - {}", kernel_link_path.display());
    
    // Generate device tree
    if let Some(dts_path) = device_tree::find_device_tree_file(board_name) {
        if let Ok(dts_content) = fs::read_to_string(&dts_path) {
            if let Ok(device_tree_info) = device_tree::generate_device_tree_info(&dts_content) {
                fs::write(&common_dt_path, &device_tree_info).unwrap_or_default();
                println!("Generated device tree:");
                println!("  - {}", common_dt_path.display());
            }
        }
    }
    
    println!("Configuration generated successfully!");
}

/// Build boot/kernel
/// 构建 boot/kernel
pub fn build(board_name: &str, target: &str) {
    // First generate configuration
    generate_config(board_name);
    
    let root = root_path::get_root_path();
    
    // Get board config
    let config_path = match config::find_board_config(board_name, &root) {
        Some(path) => path,
        None => {
            utils::print_error(&format!("Board '{}' not found", board_name));
            return;
        }
    };
    
    let board_config = match config::parse_board_config(&config_path) {
        Some(cfg) => cfg,
        None => {
            utils::print_error("Failed to parse board configuration");
            return;
        }
    };
    
    let target_arch = config::get_target_arch(&board_config.cpu_core);
    let arch_features = config::get_arch_features(&board_config.cpu_core);
    let features_arg = arch_features.join(",");
    
    let root_path = PathBuf::from(&root);
    
    // Determine build target
    let build_target = if target == "boot" || target == "Boot" || target == "BOOT" {
        "boot"
    } else if target == "kernel" || target == "Kernel" || target == "KERNEL" {
        "kernel"
    } else {
        "all"
    };
    
    // Build bootloader
    if build_target == "all" || build_target == "boot" {
        println!("Building bootloader...");
        
        let mut args = Vec::new();
        args.push("build");
        args.push("--release");
        args.push("--target");
        args.push(&target_arch);
        if !features_arg.is_empty() {
            args.push("--features");
            args.push(&features_arg);
        }
        
        let args_ref: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        
        let status = Command::new("cargo")
            .args(&args_ref)
            .current_dir(root_path.join("boot"))
            .status();
        
        match status {
            Ok(s) if s.success() => utils::print_success("Bootloader built successfully!"),
            Ok(_) => utils::print_error("Bootloader build failed"),
            Err(e) => utils::print_error(&format!("Failed to run cargo: {}", e)),
        }
    }
    
    // Build kernel
    if build_target == "all" || build_target == "kernel" {
        println!("Building kernel...");
        
        let mut args = Vec::new();
        args.push("build");
        args.push("--release");
        args.push("--target");
        args.push(&target_arch);
        if !features_arg.is_empty() {
            args.push("--features");
            args.push(&features_arg);
        }
        
        let args_ref: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        
        let status = Command::new("cargo")
            .args(&args_ref)
            .current_dir(root_path.join("kernel"))
            .status();
        
        match status {
            Ok(s) if s.success() => utils::print_success("Kernel built successfully!"),
            Ok(_) => utils::print_error("Kernel build failed"),
            Err(e) => utils::print_error(&format!("Failed to run cargo: {}", e)),
        }
    }
}

/// Clean build artifacts
/// 清理构建产物
pub fn clean_build() {
    let root = root_path::get_root_path();
    let root_path = PathBuf::from(&root);
    
    utils::print_info("Cleaning build...");
    
    // Clean linker scripts
    let _ = fs::remove_file(root_path.join("boot/link.x"));
    let _ = fs::remove_file(root_path.join("kernel/link.x"));
    let _ = fs::remove_file(root_path.join("common/src/generated/devicetree.rs"));
    
    // Clean cargo build
    let _ = Command::new("cargo")
        .arg("clean")
        .current_dir(root_path.join("boot"))
        .status();
    
    let _ = Command::new("cargo")
        .arg("clean")
        .current_dir(root_path.join("kernel"))
        .status();
    
    utils::print_success("Build cleaned!");
}
