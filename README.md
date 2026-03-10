# FeatherCore

A Rust-based Unix-like Real-Time Operating System (RTOS) designed for embedded systems.

## Features

- **Rust Implementation**: Built entirely in Rust for safety and performance
- **Unix-like API**: Provides familiar Unix system calls and abstractions
- **Real-Time Capabilities**: Preemptive scheduling with configurable priorities
- **Multi-Architecture Support**:
  - ARMv7-M (Cortex-M3/M4/M7)
  - ARMv8-M (Cortex-M23/M33)
  - ARMv7-A (Cortex-A5/A7/A9)
  - RISC-V (RV32/RV64)
- **Memory Management**: Dynamic memory allocation with configurable allocators
- **File System Support**: Virtual File System (VFS) with support for various file systems
- **Networking**: TCP/IP stack with socket API
- **POSIX Compliance**: Partial POSIX compliance for application portability

## Architecture

FeatherCore follows a modular architecture:

- **Kernel**: Core functionality including scheduling, memory management, and interrupt handling
- **Architecture Support**: Hardware-specific code for different CPU architectures
- **Drivers**: Device drivers for various peripherals
- **File System**: VFS and file system implementations
- **Networking**: TCP/IP stack and network interfaces
- **POSIX Layer**: POSIX API implementation

## Getting Started

### Prerequisites

- Rust compiler with nightly toolchain
- Cargo
- Cross-compilation toolchains for target architectures
- QEMU for simulation (optional)

### Building

FeatherCore 使用统一的构建工具来管理配置生成和构建流程。构建工具 (`feathercore-build`) 在编译时自动生成，确保适用于当前主机平台。

#### 1. 安装依赖
```bash
# 安装 Rust 工具链（用于编译构建工具）
rustup default stable

# 安装嵌入式目标工具链
rustup target add thumbv7m-none-eabi
rustup target add thumbv8m.main-none-eabihf
rustup target add riscv32imc-unknown-none-elf

# 安装 ARM 工具链（可选，用于生成二进制文件）
sudo apt-get install gcc-arm-none-eabi
```

#### 2. 构建流程
```bash
# 1. 编译构建工具（适用于当前主机平台）
rustc build_tool.rs -o feathercore-build

# 2. 使用构建工具
./feathercore-build --help                    # 查看帮助
./feathercore-build list-boards               # 列出支持的板级
./feathercore-build generate stm32            # 生成板级配置
./feathercore-build build stm32 all           # 构建所有目标
./feathercore-build clean                     # 清理构建输出
```

#### 3. 构建目标选项
- `all` - 构建 bootloader 和 kernel（默认）
- `boot` - 仅构建 bootloader
- `kernel` - 仅构建 kernel

### 详细构建文档
- [快速开始](docs/QUICKSTART.md) - 构建系统使用指南
- [构建工具使用说明](docs/BUILD_TOOL_USAGE.md) - 新构建工具详细说明
- [项目总结](docs/PROJECT_SUMMARY.md) - 项目结构和文件功能分析
- [项目状态](docs/PROJECT_STATUS.md) - 项目结构和技术栈

### Running

```bash
# Run on QEMU (example for ARM Cortex-M3)
qemu-system-arm -M lm3s6965evb -cpu cortex-m3 -kernel target/thumbv7m-none-eabi/debug/feathercore -nographic
```

## Project Structure

```
FeatherCore/
├── build_tool.rs        # 构建工具源代码（编译时生成可执行文件）
├── Cargo.toml           # 项目配置文件
├── .cargo/config.toml   # Cargo工具链配置
├── arch/                # 架构支持代码
│   ├── arm/            # ARM架构支持
│   └── riscv/          # RISC-V架构支持
├── board/               # 板级配置文件
│   ├── stm32/          # STM32系列板级配置
│   ├── nxp/            # NXP系列板级配置
│   ├── renesas/        # Renesas系列板级配置
│   └── esp/            # ESP系列板级配置
├── boot/                # Bootloader项目
│   ├── Cargo.toml      # bootloader项目配置
│   ├── link.x          # 构建时生成的链接脚本
│   └── src/            # bootloader源代码
├── kernel/              # Kernel项目
│   ├── Cargo.toml      # kernel项目配置
│   ├── link.x          # 构建时生成的链接脚本
│   └── src/            # kernel源代码
└── docs/                # 文档目录
    ├── BUILD_TOOL_USAGE.md    # 构建工具使用说明
    ├── PROJECT_SUMMARY.md     # 项目总结
    ├── QUICKSTART.md          # 快速开始指南
    └── README.md              # 文档说明
```

## Documentation

- [Architecture Guide](docs/architecture.md)
- [Porting Guide](docs/porting.md)
- [API Reference](docs/api.md)
- [Contributing Guide](docs/contributing.md)

## License

FeatherCore is dual-licensed under the MIT License and Apache License 2.0.

## Contributing

Contributions are welcome! Please see the [Contributing Guide](docs/contributing.md) for more information.
