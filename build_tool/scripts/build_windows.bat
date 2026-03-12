@echo off
REM FeatherCore Build Tool - Windows Build Script
REM 用于在 Windows 平台编译 build_tool

echo FeatherCore Build Tool - Windows Build Script
echo ========================================

REM Check if Rust is installed
where rustc >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Rust is not installed. Please install Rust first.
    echo You can install Rust from https://rustup.rs/
    exit /b 1
)

where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Error: Cargo is not installed. Please install Rust first.
    echo You can install Rust from https://rustup.rs/
    exit /b 1
)

REM Show Rust version
echo Rust version: 
rustc --version
echo Cargo version: 
cargo --version

REM Get script directory
set SCRIPT_DIR=%~dp0
set BUILD_TOOL_DIR=%SCRIPT_DIR%

cd /d "%BUILD_TOOL_DIR%"

echo.
echo Building build_tool...

REM Compile build_tool
cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo Build successful!
    echo feathercore-build executable created at target\release\feathercore-build.exe
    echo.
    echo Usage:
    echo   .\target\release\feathercore-build.exe -r C:\path\to\FeatherCore list-boards
    echo   .\target\release\feathercore-build.exe -r C:\path\to\FeatherCore generate stm32f429i-disc
    echo   .\target\release\feathercore-build.exe -r C:\path\to\FeatherCore build stm32f429i-disc all
    echo   .\target\release\feathercore-build.exe -r C:\path\to\FeatherCore clean
) else (
    echo.
    echo Build failed!
    exit /b 1
)
