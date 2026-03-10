# FeatherCore 工程说明

## 1. 项目概述

FeatherCore是一个轻量级的嵌入式实时操作系统内核，支持ARM和RISC-V架构，适用于各种嵌入式设备。

## 2. 目录结构

```
FeatherCore/
├── .cargo/            # Cargo配置文件
├── Cargo.lock         # 依赖版本锁定文件
├── Cargo.toml         # 主项目配置文件
├── README.md          # 项目说明文档
├── arch/              # 架构相关代码
│   ├── arm/           # ARM架构支持
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   └── riscv/         # RISC-V架构支持
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
├── board/             # 板级支持包
│   └── stm32/         # STM32系列板卡支持
├── boot/              # 引导加载程序
│   ├── link.x         # 引导加载程序链接脚本（自动生成）
│   └── stm32/         # STM32引导加载程序实现
├── docs/              # 工程文档
├── host_build.rs      # 主机构建脚本（构建时执行）
├── host_build_linkers.rs  # 主机端链接脚本生成器
├── kernel/            # 内核核心代码
│   ├── Cargo.toml
│   ├── link.x         # 内核链接脚本（自动生成）
│   └── src/
│       ├── future.rs
│       ├── irq.rs
│       ├── lib.rs
│       ├── log.rs
│       ├── mm.rs
│       ├── sched.rs
│       ├── sync.rs
│       ├── task.rs
│       └── time.rs
└── src/               # 主源码目录
    ├── arch.rs
    └── lib.rs
```

## 3. 主要目录说明

### 3.1 arch/
包含不同架构的实现代码，目前支持ARM和RISC-V架构。

### 3.2 board/
板级支持包，包含特定硬件平台的配置和实现。

### 3.3 boot/
引导加载程序代码，负责系统初始化和内核加载。

### 3.4 kernel/
内核核心代码，包含任务调度、内存管理、中断处理等核心功能。

## 4. 构建方式

### 4.1 环境要求
- Rust编译器 (nightly版本)
- cargo (Rust构建工具)
- 目标架构的交叉编译工具链

### 4.2 构建步骤

1. **构建项目**
   ```bash
   cargo build --features stm32f429i-disc
   ```

2. **指定目标架构构建**
   ```bash
   cargo build --target thumbv7m-none-eabi --features stm32f429i-disc
   ```

### 4.3 清理构建
   ```bash
   cargo clean
   ```

## 5. 主机端工具

项目中以`host_`开头的Rust源码文件是在主机端编译执行的工具：

| 文件名 | 说明 |
|-------|------|
| `host_build.rs` | 主构建脚本，负责调用链接脚本生成器 |
| `host_build_linkers.rs` | 链接脚本生成器，根据板级配置生成boot和kernel的链接脚本 |

## 6. 板级配置

板级配置文件位于`board/<arch>/boards/<vendor>/<board>/config/board.toml`，包含以下主要配置项：

- **board**: 板卡基本信息
- **cpu**: CPU配置
- **memory**: 内存配置
- **bootloader**: 引导加载程序配置
- **kernel**: 内核配置
- **pinout**: 引脚配置

## 7. 链接脚本生成

构建过程中，`host_build_linkers.rs`会根据板级配置文件自动生成：
- **boot/link.x**: 引导加载程序链接脚本
- **kernel/link.x**: 内核链接脚本

这些链接脚本定义了内存布局、代码段分配等关键信息。

## 8. 架构支持

目前支持的架构：
- ARM Cortex-M系列 (thumbv7m, thumbv7em, thumbv8m)
- RISC-V系列 (riscv32, riscv64)

## 9. 开发流程

1. 选择或创建板级配置文件
2. 修改或扩展内核功能
3. 构建项目
4. 烧录到目标硬件
5. 调试和测试

## 10. 注意事项

- 项目使用Rust nightly版本，需要确保安装了正确的工具链
- 构建特定板卡时需要启用对应的feature
- 链接脚本由构建脚本自动生成，无需手动修改
- 主机端工具仅在构建过程中使用，不会被编译到目标固件中

## 11. 项目状态

详细的整理状态请参考 [PROJECT_STATUS.md](PROJECT_STATUS.md)。
