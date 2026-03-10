# FeatherCore 快速开始

## 构建系统概述

FeatherCore 使用统一的构建工具来管理配置生成和构建流程。构建工具 (`feathercore-build`) 在编译时自动生成，确保适用于当前主机平台。

## 安装依赖

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

## 快速开始

### 1. 编译构建工具
```bash
# 编译适用于当前主机平台的构建工具
rustc build_tool.rs -o feathercore-build
```

### 2. 查看支持的板级
```bash
# 列出所有可用的开发板
./feathercore-build list-boards
```

### 3. 构建 STM32 目标
```bash
# 为 STM32 生成配置
./feathercore-build generate stm32

# 构建所有目标（bootloader 和 kernel）
./feathercore-build build stm32 all
```

### 4. 清理构建
```bash
# 清理所有构建输出
./feathercore-build clean
```

## 详细使用说明

### 构建工具命令
```bash
# 显示帮助信息
./feathercore-build --help

# 显示版本信息
./feathercore-build --version

# 列出支持的板级
./feathercore-build list-boards

# 显示板级详细信息
./feathercore-build show-board <板级名称>

# 生成板级配置
./feathercore-build generate <板级名称>

# 构建目标
./feathercore-build build <板级名称> <构建目标>

# 清理构建
./feathercore-build clean
```

### 构建目标选项
- `all` - 构建 bootloader 和 kernel（默认）
- `boot` - 仅构建 bootloader
- `kernel` - 仅构建 kernel

## 支持的板级

当前支持的板级可以通过 `list-boards` 命令查看。板级配置文件位于 `board/` 目录下，支持以下目录结构：

```
board/
├── stm32/     # STM32系列板级配置
├── nxp/       # NXP系列板级配置
├── renesas/   # Renesas系列板级配置
└── esp/       # ESP系列板级配置
```

## 构建流程说明

1. **编译构建工具**：首先编译 `build_tool.rs` 生成适用于当前主机平台的 `feathercore-build` 可执行文件
2. **配置生成**：使用构建工具读取板级配置文件，生成链接脚本 (`boot/link.x` 和 `kernel/link.x`)
3. **镜像构建**：构建 bootloader 和/或 kernel 镜像
4. **清理**：可以清理所有生成的文件，只保留源代码

## 故障排除

### 常见问题

1. **构建工具编译失败**
   - 确保已安装 Rust 工具链：`rustc --version`
   - 检查 `build_tool.rs` 文件是否存在

2. **板级未找到**
   - 使用 `list-boards` 命令查看支持的板级
   - 确保板级配置文件位于正确的目录结构下

3. **构建失败**
   - 检查是否已安装目标架构工具链
   - 查看构建日志中的错误信息

## 下一步

1. **查看构建工具详细说明**: [BUILD_TOOL_USAGE.md](BUILD_TOOL_USAGE.md)
2. **了解项目结构**: [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)
3. **开始开发**: 修改 `boot/src/main.rs` 或 `kernel/src/main.rs`
4. **添加新板级**: 在 `board/` 目录下创建新的板级配置文件