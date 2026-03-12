# FeatherCore 构建指南

## 概述

FeatherCore 是一个基于 Rust 开发的类 Unix 实时操作系统 (RTOS)，专为嵌入式系统设计。构建系统采用分层架构，支持多种芯片平台和开发板。

## 构建架构

```
┌─────────────────────────────────────────────────────────────────┐
│                        构建主机 (Host)                           │
│                                                                 │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │              build_tool (std - 主机工具)                │   │
│   │  • 解析 platform 目录下的配置文件                        │   │
│   │  • 生成链接脚本 (link.x)                                │   │
│   │  • 解析设备树文件                                        │   │
│   │  • 启动 boot/kernel 构建                                │   │
│   └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     FeatherCore 根目录                           │
│                    (/FeatherCore/FeatherCore)                    │
│                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐     │
│  │   platform   │  │    build     │  │      boot        │     │
│  │   (配置)     │──│    (工具)    │──│   (no_std)       │     │
│  │              │  │              │  │                  │     │
│  │ board/       │  │ build_tool/  │  │ 链接脚本生成位置  │     │
│  │ chip/        │  │ scripts/     │  │ target/          │     │
│  └──────────────┘  └──────────────┘  └──────────────────┘     │
│                                                                 │
│  ┌──────────────┐  ┌──────────────┐                            │
│  │    kernel    │  │    common    │                            │
│  │   (no_std)   │──│   (no_std)   │                            │
│  │              │  │              │                            │
│  │ 链接脚本生成位置  │ 共享库       │                            │
│  │ target/      │  │ arch/        │                            │
│  └──────────────┘  └──────────────┘                            │
└─────────────────────────────────────────────────────────────────┘
```

## 构建流程详解

### 步骤 1: 编译 build_tool 主机工具

**目的**: 编译生成一个可在主机上运行的构建工具，用于解析配置文件和生成构建产物。

**执行命令**:
```bash
cd /home/uan/develop/FeatherCore/FeatherCore/build_tool
cargo build --release
```

**生成产物**:
- `target/release/feathercore-build` - 构建工具可执行文件

**脚本方式**:
```bash
cd /home/uan/develop/FeatherCore/FeatherCore/build_tool
./scripts/build_linux.sh    # Linux
# 或
.\scripts\build_windows.bat # Windows
```

---

### 步骤 2: 使用 build_tool 解析配置文件

**目的**: 解析 platform 目录下的板级配置文件和设备树，生成链接脚本和设备树信息。

**关键参数**:
- `-r/--root`: FeatherCore 根目录路径（必需）
- `BOARD_NAME`: 开发板名称

**执行命令**:
```bash
cd /home/uan/develop/FeatherCore/FeatherCore/build_tool
./target/release/feathercore-build -r /home/uan/develop/FeatherCore/FeatherCore generate stm32f429i-disc
```

**解析过程**:

1. **查找配置文件**: 在 `platform/board/` 目录下查找指定开发板的配置文件
   - 路径格式: `platform/board/<厂商>/boards/vendor/<开发板名>/config/board.toml`
   
2. **解析 TOML 配置**: 提取以下关键配置:
   - `bootloader.boot_base_address` / `bootloader.code_base` - Boot 起始地址
   - `bootloader.boot_size` / `bootloader.code_size` - Boot 大小
   - `kernel.kernel_base_address` / `kernel.code_base` - Kernel 起始地址
   - `kernel.kernel_size` / `kernel.code_size` - Kernel 大小
   - `cpu.core` - CPU 核心类型 (如 cortex-m4)
   - `cpu.fpu` - 是否启用 FPU

3. **确定架构类型**:
   - 根据 `cpu.core` 确定目标架构 (如 thumbv7em-none-eabihf)

4. **生成链接脚本**: 根据架构和内存配置生成链接脚本

**生成产物**:
- `boot/link.x` - Boot 链接脚本
- `kernel/link.x` - Kernel 链接脚本
- `common/src/generated/devicetree.rs` - 设备树信息

---

### 步骤 3: 构建 Boot 镜像

**目的**: 编译生成 Bootloader 镜像。

**执行命令**:
```bash
cd /home/uan/develop/FeatherCore/FeatherCore/build_tool
./target/release/feathercore-build -r /home/uan/develop/FeatherCore/FeatherCore build stm32f429i-disc boot
```

**构建过程**:

1. **自动执行 generate**: 首先自动执行配置生成步骤

2. **确定构建参数**:
   - 目标架构: `thumbv7em-none-eabihf` (根据 cpu.core = cortex-m4)
   - 架构特性: `armv7-em` (根据 cortex-m4)

3. **执行 cargo 构建**:
   ```bash
   cd /home/uan/develop/FeatherCore/FeatherCore/boot
   cargo build --release \
       --target thumbv7em-none-eabihf \
       --features armv7-em
   ```

4. **依赖链**:
   ```
   feathercore-boot (bin)
       └── feathercore-common (lib, no_std)
           ├── feathercore-arch-arm (lib, no_std)
           └── alloc (内置)
   ```

**生成产物**:
- `boot/target/thumbv7em-none-eabihf/release/feathercore-boot`

---

### 步骤 4: 构建 Kernel 镜像

**目的**: 编译生成内核镜像。

**执行命令**:
```bash
cd /home/uan/develop/FeatherCore/FeatherCore/build_tool
./target/release/feathercore-build -r /home/uan/develop/FeatherCore/FeatherCore build stm32f429i-disc kernel
```

**构建过程** (与 Boot 类似):

1. **自动执行 generate**: 首先自动执行配置生成步骤

2. **确定构建参数**:
   - 目标架构: `thumbv7em-none-eabihf`
   - 架构特性: `armv7-em`

3. **执行 cargo 构建**:
   ```bash
   cd /home/uan/develop/FeatherCore/FeatherCore/kernel
   cargo build --release \
       --target thumbv7em-none-eabihf \
       --features armv7-em
   ```

**生成产物**:
- `kernel/target/thumbv7em-none-eabihf/release/feathercore-kernel`

---

## 一键构建命令

### 完整构建 (Boot + Kernel)
```bash
cd /home/uan/develop/FeatherCore/FeatherCore/build_tool
./target/release/feathercore-build -r /home/uan/develop/FeatherCore/FeatherCore build stm32f429i-disc all
```

### 仅构建 Boot
```bash
cd /home/uan/develop/FeatherCore/FeatherCore/build_tool
./target/release/feathercore-build -r /home/uan/develop/FeatherCore/FeatherCore build stm32f429i-disc boot
```

### 仅构建 Kernel
```bash
cd /home/uan/develop/FeatherCore/FeatherCore/build_tool
./target/release/feathercore-build -r /home/uan/develop/FeatherCore/FeatherCore build stm32f429i-disc kernel
```

### 清理构建
```bash
cd /home/uan/develop/FeatherCore/FeatherCore/build_tool
./target/release/feathercore-build -r /home/uan/develop/FeatherCore/FeatherCore clean
```

---

## 支持的开发板

当前支持的开发板:
- `stm32f429i-disc` - STM32F429 Discovery
- `stm32h7s78-dk` - STM32H7S78 Discovery
- `stm32n6570-dk` - STM32N6570 Discovery
- `stm32u5a9j-dk` - STM32U5A9 Discovery
- `stm32u5g9j-dk1` - STM32U5G9J Discovery
- `mimxrt1170-evkb` - i.MX RT1170 EVKB
- `frdm-rw612` - FRDM-RW612
- `ek-ra8p1` - EK-RA8P1
- `esp32-c3-devkitc` - ESP32-C3 DevKitC
- `esp32-c5-devkitc` - ESP32-C5 DevKitC
- `esp32-c6-devkitm` - ESP32-C6 DevKitM

---

## 架构与特性映射

| CPU Core | 目标架构 | 架构特性 | FPU |
|----------|----------|----------|-----|
| cortex-m0/m0+ | thumbv6m-none-eabi | armv6-m | - |
| cortex-m3 | thumbv7m-none-eabi | armv7-m | - |
| cortex-m4 | thumbv7em-none-eabihf | armv7-em | 有 |
| cortex-m7 | thumbv7em-none-eabihf | armv7-em | 有 |
| cortex-m23 | thumbv8m.base-none-eabi | armv8-m-base | - |
| cortex-m33 | thumbv8m.main-none-eabihf | armv8-m-main | 可选 |
| cortex-m55 | thumbv8m.main-none-eabihf | armv8-m-main | 有 |
| cortex-a5/a7/a8/a9 | armv7a-none-eabihf | armv7-a | 有 |
| cortex-a53/a55/a72... | aarch64-none-elf | armv8-a | 有 |
| RISC-V | riscv32/riscv64 | riscv | - |

---

## 项目结构

```
FeatherCore/
├── platform/                  # 平台配置文件目录
│   ├── board/                # 板级配置文件
│   │   ├── stm32/
│   │   │   └── boards/vendor/stm32f429i-disc/config/board.toml
│   │   ├── nxp/
│   │   ├── renesas/
│   │   └── esp/
│   └── chip/                 # 芯片配置文件
│       ├── stm32/
│       └── ...
│
├── build_tool/               # 构建工具 (std - 主机工具)
│   ├── Cargo.toml           # 构建工具项目配置
│   ├── src/
│   │   ├── main.rs          # 主入口
│   │   ├── build.rs         # 构建逻辑
│   │   ├── config.rs        # 配置解析
│   │   ├── device_tree.rs   # 设备树解析
│   │   ├── linker.rs        # 链接脚本生成
│   │   ├── root_path.rs    # 路径管理
│   │   └── utils.rs         # 工具函数
│   └── scripts/
│       ├── build_linux.sh   # Linux 构建脚本
│       ├── build_windows.bat # Windows 构建脚本
│       └── build.sh         # 统一构建脚本
│
├── common/                   # 公共库 (no_std)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs           # 主库入口
│   │   ├── mm.rs            # 内存管理
│   │   ├── devicetree.rs    # 设备树
│   │   ├── async_rt.rs      # 异步运行时
│   │   ├── sync.rs          # 同步原语
│   │   ├── driver/          # 驱动框架
│   │   ├── fs/              # 文件系统
│   │   ├── arch/            # 架构支持
│   │   │   ├── arm/         # ARM 架构
│   │   │   └── riscv/       # RISC-V 架构
│   │   └── generated/       # 生成的代码
│   │       └── devicetree.rs
│   └── arch/                # 架构子包
│       ├── arm/Cargo.toml
│       └── riscv/Cargo.toml
│
├── boot/                     # Bootloader (no_std)
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   └── link.x               # 链接脚本 (生成)
│
├── kernel/                   # Kernel (no_std)
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   └── link.x               # 链接脚本 (生成)
│
└── docs/                    # 文档
```

---

## 构建配置说明

### 板级配置文件格式 (board.toml)

```toml
# 芯片配置
[chip]
name = "STM32F429ZI"
vendor = "STMicroelectronics"

# CPU 配置
[cpu]
core = "cortex-m4"
fpu = true
clock_hz = 180000000

# 内存配置
[memory]
flash_base = 0x08000000
flash_size = 2048 * 1024
sram_base = 0x20000000
sram_size = 256 * 1024

# Bootloader 配置
[bootloader]
boot_base_address = 0x08000000
boot_size = 0x10000

# Kernel 配置
[kernel]
kernel_base_address = 0x08010000
kernel_size = 0x1F0000
kernel_stack_size = 0x4000
kernel_heap_size = 0x10000
```

---

## 注意事项

1. **根路径参数**: 必须使用 `-r` 或 `--root` 参数指定 FeatherCore 根目录路径

2. **no_std 构建**: common、boot、kernel 均为 no_std 库，不依赖标准库

3. **全局分配器**: common 库内置了简单的 bump allocator 作为全局分配器

4. **架构代码**: 所有架构相关的代码都放在 `arch/` 目录下

5. **设备树**: 设备树文件位于 `platform/chip/` 目录下，使用 TOML 格式
