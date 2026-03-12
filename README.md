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

- Rust compiler with stable toolchain (for building build_tool)
- Cargo
- Cross-compilation toolchains for target architectures
- QEMU for simulation (optional)

### Building

FeatherCore 使用统一的构建工具来管理配置生成和构建流程。构建工具 (`feathercore-build`) 是一个独立的 Cargo 项目，确保适用于当前主机平台。

#### 1. 安装依赖
```bash
# 安装 Rust 工具链（用于编译构建工具）
rustup default stable

# 安装嵌入式目标工具链
rustup target add thumbv7m-none-eabi
rustup target add thumbv8m.main-none-eabihf
rustup target add riscv32imc-unknown-none-elf
rustup target add thumbv7em-none-eabihf

# 安装 ARM 工具链（可选，用于生成二进制文件）
sudo apt-get install gcc-arm-none-eabi
```

#### 2. 构建流程

**方式一：直接使用构建工具**
```bash
# 1. 编译构建工具（适用于当前主机平台）
cd build_tool
cargo build --release

# 2. 使用构建工具（必须指定 -r 参数指向 FeatherCore 根目录）
cd ..
./build_tool/target/release/feathercore-build -r /home/uan/develop/FeatherCore/FeatherCore list-boards
./build_tool/target/release/feathercore-build -r /home/uan/develop/FeatherCore/FeatherCore generate stm32f429i-disc
./build_tool/target/release/feathercore-build -r /home/uan/develop/FeatherCore/FeatherCore build stm32f429i-disc all
./build_tool/target/release/feathercore-build -r /home/uan/develop/FeatherCore/FeatherCore clean
```

**方式二：使用构建脚本**
```bash
# Linux 平台
cd build_tool
./scripts/build.sh -r /home/uan/develop/FeatherCore/FeatherCore build stm32f429i-disc all

# Windows 平台
cd build_tool
.\scripts\build_windows.bat
```

**方式三：使用 Linux 构建脚本**
```bash
cd build_tool
./scripts/build_linux.sh
```

#### 3. 构建目标选项
- `all` - 构建 bootloader 和 kernel（默认）
- `boot` - 仅构建 bootloader
- `kernel` - 仅构建 kernel

#### 4. 构建产物位置
- Boot 镜像: `boot/target/<target>/release/feathercore-boot`
- Kernel 镜像: `kernel/target/<target>/release/feathercore-kernel`

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
├── platform/            # 平台配置目录（非 Cargo 管理）
│   ├── board/           # 板级配置文件和设备树
│   │   ├── stm32/       # STM32系列板级配置
│   │   ├── nxp/         # NXP系列板级配置
│   │   ├── renesas/     # Renesas系列板级配置
│   │   └── esp/         # ESP系列板级配置
│   └── chip/            # 芯片配置文件和设备树
├── build_tool/          # 构建工具项目（Cargo 管理）
│   ├── Cargo.toml       # 构建工具项目配置
│   ├── src/             # 构建工具源代码
│   └── scripts/         # 平台构建脚本
│       ├── build_linux.sh    # Linux 构建脚本
│       └── build_windows.bat # Windows 构建脚本
├── common/              # 公共库项目（Cargo 管理）
│   ├── Cargo.toml       # 公共库项目配置
│   └── src/             # 公共库源代码
├── boot/                # Bootloader项目（Cargo 管理）
│   ├── Cargo.toml       # bootloader项目配置
│   ├── link.x           # 构建时生成的链接脚本
│   └── src/             # bootloader源代码
├── kernel/              # Kernel项目（Cargo 管理）
│   ├── Cargo.toml       # kernel项目配置
│   ├── link.x           # 构建时生成的链接脚本
│   └── src/             # kernel源代码
└── docs/                # 文档目录（非 Cargo 管理）
    ├── BUILD_TOOL_USAGE.md    # 构建工具使用说明
    ├── PROJECT_SUMMARY.md     # 项目总结
    ├── QUICKSTART.md          # 快速开始指南
    └── README.md              # 文档说明
```

## 构建流程说明

1. **编译构建工具**：首先执行对应平台的构建脚本，编译 build_tool 项目，生成适用于当前主机的构建工具。

2. **解析配置**：使用构建工具解析 platform 目录下的板级和芯片配置文件，包括设备树文件。

3. **生成构建环境**：构建工具根据解析的配置，更新 common、boot、kernel 三个目录的 Cargo 配置和链接脚本。

4. **构建目标镜像**：根据需求选择构建 boot 镜像、kernel 镜像或两者都构建。

## 依赖关系

- **build_tool**：主机架构的构建工具，使用 std 接口，用于解析配置文件和设备树文件。
- **platform**：非 Cargo 管理目录，存放板级和芯片配置文件，board 配置依赖于 chip 配置。
- **common**：公共库包，被 boot 和 kernel 引用，提供共享功能。
- **boot**：启动镜像，依赖 common 库。
- **kernel**：内核镜像，依赖 common 库。

## Documentation

- [Architecture Guide](docs/architecture.md)
- [Porting Guide](docs/porting.md)
- [API Reference](docs/api.md)
- [Contributing Guide](docs/contributing.md)

## License

FeatherCore is dual-licensed under the MIT License and Apache License 2.0.

## Contributing

Contributions are welcome! Please see the [Contributing Guide](docs/contributing.md) for more information.