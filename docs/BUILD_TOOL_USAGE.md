# FeatherCore 构建工具使用说明

## 概述

FeatherCore 构建工具是一个在编译时自动生成的主机可执行程序，用于根据板级配置文件生成链接脚本和编译配置，并启动 boot 或 kernel 镜像的构建。

## 构建流程

1. **编译构建工具**：首先编译 `build_tool.rs` 生成适用于当前主机平台的 `feathercore-build` 可执行文件
2. **配置生成**：使用构建工具读取板级配置文件，生成链接脚本
3. **镜像构建**：构建 bootloader 和/或 kernel 镜像

## 构建工具功能

1. **列出支持的板级** - 显示所有可用的开发板配置
2. **显示板级信息** - 查看特定开发板的详细配置
3. **生成配置** - 根据板级配置生成链接脚本
4. **构建镜像** - 构建 bootloader 和/或 kernel
5. **清理构建** - 清理所有构建输出

## 使用方法

### 1. 编译构建工具
```bash
# 编译构建工具（适用于当前主机平台）
rustc build_tool.rs -o feathercore-build
```

### 2. 使用构建工具
```bash
# 显示帮助信息
./feathercore-build --help

# 显示版本信息
./feathercore-build --version

# 列出支持的板级
./feathercore-build list-boards

# 显示板级信息
./feathercore-build show-board <板级名称>

# 生成配置
./feathercore-build generate <板级名称>

# 构建镜像
./feathercore-build build <板级名称> <构建目标>

# 清理构建
./feathercore-build clean
```

### 构建目标

- `all` - 构建 bootloader 和 kernel（默认）
- `boot` - 仅构建 bootloader
- `kernel` - 仅构建 kernel

### 3. 完整示例

```bash
# 1. 编译构建工具
rustc build_tool.rs -o feathercore-build

# 2. 查看所有支持的板级
./feathercore-build list-boards

# 3. 查看 STM32 开发板配置
./feathercore-build show-board stm32

# 4. 为 STM32 生成配置
./feathercore-build generate stm32

# 5. 构建 STM32 的 bootloader 和 kernel
./feathercore-build build stm32 all

# 6. 仅构建 bootloader
./feathercore-build build stm32 boot

# 7. 仅构建 kernel
./feathercore-build build stm32 kernel

# 8. 清理所有构建输出
./feathercore-build clean
```

## 工作流程

1. **配置生成**：工具读取板级配置文件（位于 `board/` 目录下），解析配置参数，生成相应的链接脚本
2. **链接脚本生成**：在 `boot/` 和 `kernel/` 目录下生成 `link.x` 文件
3. **构建执行**：调用 cargo 构建相应的项目
4. **输出文件**：构建结果位于各项目的 `target/` 目录下

## 支持的板级

当前支持的板级可以通过 `list-boards` 命令查看。板级配置文件位于 `board/` 目录下，支持以下目录结构：

```
board/
├── stm32/
│   └── boards/
│       └── vendor/
│           ├── stm32f429i-disc/
│           │   └── config/
│           │       └── board.toml
│           └── stm32n6570-dk/
│               └── config/
│                   └── board.toml
├── nxp/
├── renesas/
└── esp/
```

## 配置文件格式

板级配置文件使用 TOML 格式，包含以下主要部分：

```toml
[board]
name = "板级名称"
vendor = "厂商"
family = "系列"
mcu = "MCU型号"

[cpu]
core = "CPU核心"
frequency = 频率值
fpu = true/false

[memory]
flash_size = Flash大小
sram_size = SRAM大小

[bootloader]
code_base = 代码基地址
code_size = 代码大小
stack_size = 栈大小

[kernel]
code_base = 代码基地址
code_size = 代码大小
stack_size = 栈大小
heap_size = 堆大小

[pinout]
引脚定义...
```

## 生成的链接脚本

构建工具会生成两个链接脚本：

1. **boot/link.x** - Bootloader 链接脚本
2. **kernel/link.x** - Kernel 链接脚本

这些链接脚本根据板级配置文件中的内存布局参数生成，确保代码正确放置在 Flash 和 RAM 中。

## 注意事项

1. **构建工具编译**：`feathercore-build` 需要在每次构建前编译，确保适用于当前主机平台
2. **Rust 工具链**：需要 Rust 工具链（rustc）来编译构建工具
3. **目标架构工具链**：需要安装相应的目标架构工具链（如 `thumbv7m-none-eabi`）来构建嵌入式目标
4. **网络连接**：首次构建可能需要下载依赖，请确保网络连接正常
5. **清理构建**：清理构建会删除所有构建输出和生成的链接脚本，但会保留 `build_tool.rs` 源代码

## 故障排除

### 常见问题

1. **板级未找到**：确保板级名称正确，且配置文件位于正确的目录结构下
2. **构建失败**：检查 Rust 工具链是否安装，目标架构工具链是否可用
3. **权限问题**：确保对相关目录有读写权限

### 调试信息

使用构建工具时，可以查看生成的链接脚本内容，确保内存布局正确。如果遇到问题，可以检查：

1. 板级配置文件是否正确
2. 生成的链接脚本是否包含正确的内存地址和大小
3. 构建日志中的错误信息