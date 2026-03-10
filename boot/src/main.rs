//! FeatherCore Bootloader
//! 
//! This is the bootloader binary that initializes hardware and loads the kernel.

#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(panic_info_message)]

use core::panic::PanicInfo;

/// Bootloader entry point
#[no_mangle]
pub extern "C" fn boot_main() -> ! {
    // Initialize minimal hardware
    init_hardware();
    
    // Load kernel from storage
    load_kernel();
    
    // Jump to kernel
    jump_to_kernel();
    
    // Should never reach here
    loop {}
}

/// Initialize minimal hardware required for bootloading
fn init_hardware() {
    // TODO: Initialize clocks, GPIOs, and basic peripherals
    // This will be board-specific
}

/// Load kernel from storage (flash, SD card, etc.)
fn load_kernel() {
    // TODO: Implement kernel loading logic
    // This will depend on the storage medium
}

/// Jump to kernel entry point
fn jump_to_kernel() {
    // TODO: Set up kernel environment and jump to kernel entry
    // This includes setting up stack pointer, vector table, etc.
}

/// Panic handler for bootloader
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Simple panic handler for bootloader
    // In a real implementation, this would log to serial or blink LEDs
    loop {}
}

/// Language items required for no_std
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/// Entry point wrapper
#[no_mangle]
pub extern "C" fn _start() -> ! {
    boot_main()
}

/// Stack top for bootloader
#[link_section = ".stack_top"]
static STACK_TOP: [u8; 4096] = [0; 4096];