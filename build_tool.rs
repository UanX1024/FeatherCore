//! FeatherCore 构建工具
//! 
//! 主机可执行程序，负责：
//! 1. 根据板级配置文件生成链接脚本和编译配置
//! 2. 启动 boot 或 kernel 镜像的构建

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;

const VERSION: &str = "0.1.0";

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    match args[1].as_str() {
        "-h" | "--help" => print_help(),
        "-v" | "--version" => print_version(),
        "list-boards" => list_boards(),
        "show-board" => {
            if args.len() > 2 {
                show_board_info(&args[2]);
            } else {
                println!("Error: Board name required");
                print_help();
            }
        }
        "generate" => {
            let board_name = if args.len() > 2 { &args[2] } else { "stm32f429i-disc" };
            generate_config(board_name);
        }
        "build" => {
            let board_name = if args.len() > 2 { &args[2] } else { "stm32f429i-disc" };
            let target = if args.len() > 3 { args[3].as_str() } else { "all" };
            build(board_name, target);
        }
        "clean" => clean_build(),
        _ => {
            println!("Error: Unknown command '{}'", args[1]);
            print_help();
        }
    }
}

fn print_help() {
    println!("FeatherCore Build Tool v{}", VERSION);
    println!();
    println!("Usage: feathercore-build [COMMAND] [OPTIONS]");
    println!();
    println!("Commands:");
    println!("  list-boards                    List supported boards");
    println!("  show-board <NAME>              Show board information");
    println!("  generate [BOARD]               Generate configuration for board");
    println!("  build [BOARD] [all|boot|kernel] Build for board");
    println!("  clean                          Clean build");
    println!("  -h, --help                     Print this help");
    println!("  -v, --version                  Print version");
    println!();
    println!("Examples:");
    println!("  feathercore-build list-boards");
    println!("  feathercore-build show-board stm32f429i-disc");
    println!("  feathercore-build generate stm32f429i-disc");
    println!("  feathercore-build build stm32f429i-disc all");
    println!("  feathercore-build clean");
}

fn print_version() {
    println!("FeatherCore Build Tool v{}", VERSION);
}

fn list_boards() {
    let board_dir = PathBuf::from("board");
    
    println!("Supported boards:");
    
    // 查找所有板级配置文件
    let mut boards = Vec::new();
    
    if let Ok(entries) = fs::read_dir(board_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let vendor_dir = entry.path();
                let boards_dir = vendor_dir.join("boards/vendor");
                
                if boards_dir.exists() {
                    if let Ok(vendor_entries) = fs::read_dir(boards_dir) {
                        for vendor_entry in vendor_entries.flatten() {
                            if vendor_entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                                let board_name = vendor_entry.file_name().to_string_lossy().to_string();
                                boards.push(board_name);
                            }
                        }
                    }
                }
            }
        }
    }
    
    // 去重并排序
    boards.sort();
    boards.dedup();
    
    for board in &boards {
        println!("  - {}", board);
    }
    
    if boards.is_empty() {
        println!("  No boards found");
    }
}

fn show_board_info(board_name: &str) {
    let config_path = find_board_config(board_name);
    
    match config_path {
        Some(path) => {
            println!("Board: {}", board_name);
            println!("Config file: {:?}", path);
            
            // 读取配置文件
            if let Ok(content) = fs::read_to_string(&path) {
                println!("Configuration:");
                for line in content.lines() {
                    let line = line.trim();
                    if !line.is_empty() && !line.starts_with('#') {
                        println!("  {}", line);
                    }
                }
            } else {
                println!("Error: Could not read config file");
            }
        }
        None => {
            println!("Error: Board '{}' not found", board_name);
            println!("Available boards:");
            list_boards();
        }
    }
}

fn find_board_config(board_name: &str) -> Option<PathBuf> {
    let board_dir = PathBuf::from("board");
    
    // 搜索所有可能的路径
    let patterns = vec![
        format!("{}/config/board.toml", board_name),
        format!("{}/board.toml", board_name),
        format!("vendor/{}/config/board.toml", board_name),
        format!("vendor/{}/board.toml", board_name),
        // 支持嵌套目录结构
        format!("*/boards/vendor/{}/config/board.toml", board_name),
        format!("*/boards/vendor/{}/board.toml", board_name),
    ];
    
    for pattern in patterns {
        let path = board_dir.join(pattern);
        if path.exists() {
            return Some(path);
        }
    }
    
    // 递归搜索所有子目录
    if let Ok(entries) = fs::read_dir(board_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let vendor_dir = entry.path().join("boards/vendor").join(board_name).join("config/board.toml");
                if vendor_dir.exists() {
                    return Some(vendor_dir);
                }
            }
        }
    }
    
    None
}

fn generate_config(board_name: &str) {
    println!("Generating configuration for {}...", board_name);
    
    let config_path = find_board_config(board_name);
    
    match config_path {
        Some(path) => {
            // 确保目录存在
            fs::create_dir_all("boot").unwrap_or_default();
            fs::create_dir_all("kernel").unwrap_or_default();
            
            // 读取配置文件
            if let Ok(content) = fs::read_to_string(&path) {
                // 解析TOML格式配置
                let config = parse_toml_config(&content);
                
                // 获取配置值，使用默认值
                // 支持新旧变量名
                let flash_base = config.get("bootloader.boot_base_address")
                    .or_else(|| config.get("bootloader.code_base"))
                    .or_else(|| config.get("bootloader_base"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x08000000);
                
                let boot_size = config.get("bootloader.boot_size")
                    .or_else(|| config.get("bootloader.code_size"))
                    .or_else(|| config.get("bootloader_size"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x10000);
                
                let kernel_base = config.get("kernel.kernel_base_address")
                    .or_else(|| config.get("kernel.code_base"))
                    .or_else(|| config.get("kernel_base"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x08010000);
                
                let kernel_size = config.get("kernel.kernel_size")
                    .or_else(|| config.get("kernel.code_size"))
                    .or_else(|| config.get("kernel_size"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x1F0000);
                
                let stack_size = config.get("kernel.kernel_stack_size")
                    .or_else(|| config.get("kernel.stack_size"))
                    .or_else(|| config.get("kernel_stack_size"))
                    .or_else(|| config.get("stack_size"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x4000);
                
                let heap_size = config.get("kernel.kernel_heap_size")
                    .or_else(|| config.get("kernel.heap_size"))
                    .or_else(|| config.get("kernel_heap_size"))
                    .or_else(|| config.get("heap_size"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x10000);
                
                let sram_base = config.get("memory.sram_base")
                    .or_else(|| config.get("sram_base"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x20000000);
                
                let sram_size = config.get("memory.sram_size")
                    .or_else(|| config.get("sram_size"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x40000);
                
                // 生成boot链接脚本
                let boot_script = format!(r#"/* Bootloader linker script - Generated by feathercore-build */
MEMORY
{{
    FLASH (rx) : ORIGIN = 0x{:08X}, LENGTH = {}K
    SRAM (rwx) : ORIGIN = 0x{:08X}, LENGTH = {}K
}}

SECTIONS
{{
    .text : {{
        KEEP(*(.vector_table))
        *(.text*)
        *(.rodata*)
    }} > FLASH
    
    .data : {{
        _sdata = .;
        *(.data*)
        _edata = .;
    }} > SRAM AT > FLASH
    
    .bss : {{
        _sbss = .;
        *(.bss*)
        *(COMMON)
        _ebss = .;
    }} > SRAM
    
    .stack : {{
        . = ALIGN(8);
        _stack_top = .;
        . += 4K;
        _stack_bottom = .;
    }} > SRAM
}}

PROVIDE(_sidata = LOADADDR(.data));
"#,
                    flash_base,
                    boot_size / 1024,
                    sram_base,
                    sram_size / 1024
                );
                
                // 生成kernel链接脚本
                let kernel_script = format!(r#"/* Kernel linker script - Generated by feathercore-build */
MEMORY
{{
    FLASH (rx) : ORIGIN = 0x{:08X}, LENGTH = {}K
    SRAM (rwx) : ORIGIN = 0x{:08X}, LENGTH = {}K
}}

SECTIONS
{{
    .text : {{
        KEEP(*(.vector_table))
        *(.text*)
        *(.rodata*)
    }} > FLASH
    
    .data : {{
        _sdata = .;
        *(.data*)
        _edata = .;
    }} > SRAM AT > FLASH
    
    .bss : {{
        _sbss = .;
        *(.bss*)
        *(COMMON)
        _ebss = .;
    }} > SRAM
    
    .heap : {{
        . = ALIGN(8);
        _heap_start = .;
        . += {}K;
        _heap_end = .;
    }} > SRAM
    
    .stack : {{
        . = ALIGN(8);
        _stack_top = .;
        . += {}K;
        _stack_bottom = .;
    }} > SRAM
}}

PROVIDE(_sidata = LOADADDR(.data));
"#,
                    kernel_base,
                    kernel_size / 1024,
                    sram_base,
                    sram_size / 1024,
                    heap_size / 1024,
                    stack_size / 1024
                );
                
                // 写入文件
                fs::write("boot/link.x", boot_script).unwrap_or_default();
                fs::write("kernel/link.x", kernel_script).unwrap_or_default();
                
                println!("Generated linker scripts:");
                println!("  - boot/link.x");
                println!("  - kernel/link.x");
                println!("Configuration generated successfully!");
            } else {
                println!("Error: Could not read config file");
            }
        }
        None => {
            println!("Error: Board '{}' not found", board_name);
        }
    }
}

fn get_arch_features(board_name: &str) -> Vec<String> {
    let config_path = find_board_config(board_name);
    
    match config_path {
        Some(path) => {
            if let Ok(content) = fs::read_to_string(&path) {
                let config = parse_toml_config(&content);
                
                // 获取CPU核心信息
                let default_core = "cortex-m3".to_string();
                let core = config.get("cpu.core").unwrap_or(&default_core);
                
                // 根据核心信息确定架构特性
                match core.as_str() {
                    "cortex-m0" | "cortex-m0plus" => vec!["armv6-m".to_string()],
                    "cortex-m3" => vec!["armv7-m".to_string()],
                    "cortex-m4" | "cortex-m7" => vec!["armv7-em".to_string()],
                    "cortex-m23" => vec!["armv8-m-base".to_string()],
                    "cortex-m33" | "cortex-m55" | "cortex-m85" => vec!["armv8-m-main".to_string()],
                    "cortex-a5" | "cortex-a7" | "cortex-a8" | "cortex-a9" | "cortex-a15" => vec!["armv7-a".to_string()],
                    "cortex-a53" | "cortex-a55" | "cortex-a72" | "cortex-a73" | "cortex-a75" | "cortex-a76" | "cortex-a77" | "cortex-a78" => vec!["armv8-a".to_string()],
                    "cortex-a710" | "cortex-a715" | "cortex-a720" | "cortex-a520" => vec!["armv9-a".to_string()],
                    _ => vec!["armv7-m".to_string()], // 默认值
                }
            } else {
                vec!["armv7-m".to_string()] // 默认值
            }
        }
        None => {
            vec!["armv7-m".to_string()] // 默认值
        }
    }
}

fn get_target_arch(board_name: &str) -> String {
    let config_path = find_board_config(board_name);
    
    match config_path {
        Some(path) => {
            if let Ok(content) = fs::read_to_string(&path) {
                let config = parse_toml_config(&content);
                
                // 获取CPU核心信息
                let default_core = "cortex-m3".to_string();
                let default_fpu = "false".to_string();
                let core = config.get("cpu.core").unwrap_or(&default_core);
                let fpu = config.get("cpu.fpu").unwrap_or(&default_fpu) == "true";
                
                // 根据核心信息和FPU支持确定目标架构
                match core.as_str() {
                    "cortex-m0" | "cortex-m0plus" => "thumbv6m-none-eabi".to_string(),
                    "cortex-m3" => "thumbv7m-none-eabi".to_string(),
                    "cortex-m4" => if fpu { "thumbv7em-none-eabihf".to_string() } else { "thumbv7em-none-eabi".to_string() },
                    "cortex-m7" => if fpu { "thumbv7em-none-eabihf".to_string() } else { "thumbv7em-none-eabi".to_string() },
                    "cortex-m23" => "thumbv8m.base-none-eabi".to_string(),
                    "cortex-m33" => if fpu { "thumbv8m.main-none-eabihf".to_string() } else { "thumbv8m.main-none-eabi".to_string() },
                    "cortex-m55" => if fpu { "thumbv8m.main-none-eabihf".to_string() } else { "thumbv8m.main-none-eabi".to_string() },
                    "cortex-m85" => if fpu { "thumbv8m.main-none-eabihf".to_string() } else { "thumbv8m.main-none-eabi".to_string() },
                    "cortex-a5" => "armv7a-none-eabihf".to_string(),
                    "cortex-a7" => "armv7a-none-eabihf".to_string(),
                    "cortex-a8" => "armv7a-none-eabihf".to_string(),
                    "cortex-a9" => "armv7a-none-eabihf".to_string(),
                    "cortex-a15" => "armv7a-none-eabihf".to_string(),
                    "cortex-a53" => "aarch64-none-elf".to_string(),
                    "cortex-a55" => "aarch64-none-elf".to_string(),
                    "cortex-a72" => "aarch64-none-elf".to_string(),
                    "cortex-a73" => "aarch64-none-elf".to_string(),
                    "cortex-a75" => "aarch64-none-elf".to_string(),
                    "cortex-a76" => "aarch64-none-elf".to_string(),
                    "cortex-a77" => "aarch64-none-elf".to_string(),
                    "cortex-a78" => "aarch64-none-elf".to_string(),
                    _ => "thumbv7m-none-eabi".to_string(), // 默认值
                }
            } else {
                "thumbv7m-none-eabi".to_string() // 默认值
            }
        }
        None => {
            "thumbv7m-none-eabi".to_string() // 默认值
        }
    }
}

fn build(board_name: &str, target: &str) {
    println!("Building for board {}...", board_name);
    
    // 首先生成配置
    generate_config(board_name);
    
    // 确定目标架构
    let target_arch = get_target_arch(board_name);
    println!("Using target architecture: {}", target_arch);
    
    // 确定架构特性
    let arch_features = get_arch_features(board_name);
    println!("Using architecture features: {:?}", arch_features);
    
    // 确定构建目标
    let build_target = if target == "boot" || target == "Boot" || target == "BOOT" {
        "boot"
    } else if target == "kernel" || target == "Kernel" || target == "KERNEL" {
        "kernel"
    } else {
        "all"
    };
    
    // 构建bootloader
    if build_target == "all" || build_target == "boot" {
        println!("Building bootloader...");
        
        // 构建命令参数
        let mut args = Vec::new();
        args.push("build".to_string());
        args.push("--target".to_string());
        args.push(target_arch.clone());
        for feature in &arch_features {
            args.push(format!("--features={}", feature));
        }
        
        // 转换为 &str 切片
        let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        
        let status = Command::new("cargo")
            .args(&args_ref)
            .current_dir("boot")
            .status();
        
        match status {
            Ok(status) if status.success() => println!("Bootloader built successfully!"),
            Ok(_) => println!("Error: Bootloader build failed"),
            Err(e) => println!("Error: Failed to run cargo build: {}", e),
        }
    }
    
    // 构建kernel
    if build_target == "all" || build_target == "kernel" {
        println!("Building kernel...");
        
        // 构建命令参数
        let mut args = Vec::new();
        args.push("build".to_string());
        args.push("--target".to_string());
        args.push(target_arch.clone());
        for feature in &arch_features {
            args.push(format!("--features={}", feature));
        }
        
        // 转换为 &str 切片
        let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        
        let status = Command::new("cargo")
            .args(&args_ref)
            .current_dir("kernel")
            .status();
        
        match status {
            Ok(status) if status.success() => println!("Kernel built successfully!"),
            Ok(_) => println!("Error: Kernel build failed"),
            Err(e) => println!("Error: Failed to run cargo build: {}", e),
        }
    }
}

fn clean_build() {
    println!("Cleaning build...");
    
    // 清理链接脚本
    let _ = fs::remove_file("boot/link.x");
    let _ = fs::remove_file("kernel/link.x");
    
    // 清理构建输出
    let _ = Command::new("cargo")
        .arg("clean")
        .current_dir("boot")
        .status();
    
    let _ = Command::new("cargo")
        .arg("clean")
        .current_dir("kernel")
        .status();
    
    println!("Build cleaned!");
}

fn parse_toml_config(content: &str) -> HashMap<String, String> {
    let mut config = HashMap::new();
    let mut current_section = String::new();
    
    for line in content.lines() {
        let line = line.trim();
        
        // 跳过空行和注释
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        // 检查是否是章节
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }
        
        // 解析键值对
        if let Some((key, value)) = parse_key_value(line) {
            let full_key = if current_section.is_empty() {
                key.to_string()
            } else {
                format!("{}.{}", current_section, key)
            };
            config.insert(full_key, value.to_string());
        }
    }
    
    config
}

fn parse_key_value(line: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() == 2 {
        let key = parts[0].trim();
        let mut value = parts[1].trim();
        
        // 去除引号
        value = value.trim_matches('"');
        
        // 去除行内注释（# 之后的部分）
        if let Some(comment_pos) = value.find('#') {
            value = &value[..comment_pos].trim();
        }
        
        Some((key, value))
    } else {
        None
    }
}

fn parse_hex_or_decimal(s: &str) -> u32 {
    // 简化解析，不再需要处理下划线
    if s.starts_with("0x") || s.starts_with("0X") {
        u32::from_str_radix(&s[2..], 16).unwrap_or(0)
    } else {
        // 直接解析，配置文件不再使用下划线分隔数字
        s.parse().unwrap_or(0)
    }
}