#!/bin/bash
# FeatherCore Build Tool - Linux Build Script
# 用于在 Linux 平台编译 build_tool

set -e

echo "FeatherCore Build Tool - Linux Build Script"
echo "========================================"

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Error: Rust is not installed. Please install Rust first."
    echo "You can install Rust from https://rustup.rs/"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Rust first."
    echo "You can install Rust from https://rustup.rs/"
    exit 1
fi

# Show Rust version
echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_TOOL_DIR="$SCRIPT_DIR"

cd "$BUILD_TOOL_DIR"

echo ""
echo "Building build_tool..."

# Compile build_tool
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "Build successful!"
    echo "feathercore-build executable created at target/release/feathercore-build"
    echo ""
    echo "Usage:"
    echo "  ./target/release/feathercore-build -r /path/to/FeatherCore list-boards"
    echo "  ./target/release/feathercore-build -r /path/to/FeatherCore generate stm32f429i-disc"
    echo "  ./target/release/feathercore-build -r /path/to/FeatherCore build stm32f429i-disc all"
    echo "  ./target/release/feathercore-build -r /path/to/FeatherCore clean"
else
    echo ""
    echo "Build failed!"
    exit 1
fi
