# FeatherCore 架构设计文档

## 概述

FeatherCore 是一个为嵌入式系统设计的轻量级操作系统框架，采用模块化设计，支持 bootloader 和 kernel 共享公共库，同时保持两者的独立性。

## 目录结构

```
feathercore/
├── common/           # 公共库（no_std）
│   ├── async/       # 异步运行时
│   ├── mm/          # 内存管理
│   ├── sync/        # 同步原语
│   ├── driver/      # 驱动框架
│   ├── fs/          # 文件系统框架
│   └── lib.rs       # 公共库入口
├── arch/            # 架构相关代码
│   ├── arm/        # ARM 架构支持
│   └── riscv/      # RISC-V 架构支持
├── boot/            # Bootloader
├── kernel/          # 内核
└── board/           # 板级配置
```

## 设计原则

### 1. 无外部依赖
- 所有运行时代码（boot、kernel、common）都是 `no_std`
- 只有构建工具可以使用 `std`
- 不依赖任何外部 crate（除了可选的架构支持）

### 2. 代码复用
- **公共库（common）**: boot 和 kernel 共享的功能
- **架构抽象（arch）**: CPU 架构相关的代码
- **板级配置（board）**: 硬件相关的配置

### 3. 异步支持
- 在 `common/async` 中实现基础的 `no_std` 异步运行时
- **Boot**: 使用裸机版本的异步运行时
- **Kernel**: 在公共基础上添加线程调度和用户态支持

## 模块详解

### 1. 公共库（common）

#### async_rt - 异步运行时
- 提供 `no_std` 环境下的异步执行器
- 支持 `Future` trait 和简单的任务调度
- 包含 `delay` 和 `yield_now` 等实用函数

#### mm - 内存管理
- 简单的 bump 分配器（用于 bootloader）
- 内存区域描述和管理
- 页分配器接口

#### sync - 同步原语
- 自旋锁（SpinLock）
- 读写锁（RwSpinLock）
- OnceCell（一次性初始化）
- Barrier（屏障同步）

#### util - 工具函数
- 内存对齐函数
- 字节缓冲区
- 错误处理宏

#### error - 错误处理
- 统一的错误类型
- 错误处理工具

### 2. 依赖关系

```
boot → common[async,mm,sync] → arch[optional]
kernel → common[async,mm,sync,driver,fs] → arch[optional]
```

### 3. 特性（Features）系统

公共库使用特性系统来启用可选功能：

```toml
[features]
default = []
async = []      # 启用异步运行时
mm = []         # 启用内存管理
sync = []       # 启用同步原语
driver = []     # 启用驱动框架
fs = []         # 启用文件系统框架
arm = ["feathercore-arch-arm"]
riscv = ["feathercore-arch-riscv"]
```

## 使用示例

### Bootloader 使用公共库

```rust
// boot/src/main.rs
use feathercore_common::{AsyncExecutor, delay, yield_now, Result};

fn run_bootloader_tasks() -> Result<()> {
    let mut executor = AsyncExecutor::new();
    
    executor.spawn(async {
        init_storage().await?;
        verify_kernel().await?;
        prepare_kernel_env().await?;
        Ok(())
    })?;
    
    executor.run()
}
```

### Kernel 使用公共库

```rust
// kernel/src/main.rs
use feathercore_common::{AsyncExecutor, mm::MemoryMap, sync::SpinLock};

struct Kernel {
    memory_map: MemoryMap,
    lock: SpinLock<SharedData>,
}

impl Kernel {
    async fn run(&self) -> Result<()> {
        // 使用公共库的功能
        let mut executor = AsyncExecutor::new();
        // ...
    }
}
```

## 构建系统

### 工作空间配置

```toml
[workspace]
members = [
    "arch/arm",
    "arch/riscv",
    "common",
    "boot",
    "kernel",
]
```

### 构建命令

```bash
# 构建公共库
cargo build --package feathercore-common

# 构建 bootloader（针对特定板级）
cargo build --package feathercore-boot --features stm32f429i-disc

# 构建 kernel
cargo build --package feathercore-kernel --features stm32f429i-disc
```

## 扩展指南

### 添加新的公共模块

1. 在 `common/src/` 下创建新模块
2. 在 `common/Cargo.toml` 中添加对应的 feature
3. 在 `common/src/lib.rs` 中导出模块
4. 更新 boot 和 kernel 的依赖

### 添加架构支持

1. 在 `arch/` 下创建新架构目录
2. 实现必要的 trait 和函数
3. 更新公共库的依赖配置

### 添加板级支持

1. 在 `board/` 下创建板级配置
2. 更新构建工具的配置解析
3. 添加对应的 feature

## 优势

1. **代码复用**: 避免 boot 和 kernel 重复实现相同功能
2. **维护简单**: 公共功能集中维护
3. **灵活性**: 通过特性系统控制功能启用
4. **可测试性**: 公共库可以独立测试
5. **可扩展性**: 易于添加新功能和新架构

## 限制

1. **内存占用**: 公共库会增加二进制大小
2. **复杂性**: 需要仔细设计接口以避免过度耦合
3. **编译时间**: 特性系统可能增加编译复杂度

## 未来改进

1. **性能优化**: 优化异步运行时性能
2. **更多架构**: 支持更多 CPU 架构
3. **驱动框架**: 完善设备驱动框架
4. **文件系统**: 实现完整的文件系统支持
5. **安全特性**: 添加内存保护和权限管理