//! FeatherCore Bootloader
//! 
//! This is the bootloader binary that initializes hardware and loads the kernel.
//! Uses common library for async operations and utilities.

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

// Use common library
use feathercore_common::{AsyncExecutor, delay, yield_now, Result};

// Include architecture-specific startup code
#[cfg(target_arch = "arm")]
mod startup_cortex_m;

#[cfg(target_arch = "riscv32")]
mod startup_riscv;

#[cfg(target_arch = "riscv64")]
mod startup_riscv;

/// Bootloader entry point
#[no_mangle]
pub extern "C" fn boot_main() -> ! {
    // Initialize minimal hardware
    init_hardware();
    
    // Run bootloader tasks using async executor
    if let Err(_e) = run_bootloader_tasks() {
        // Handle bootloader error
        boot_panic("Bootloader error");
    }
    
    // Load kernel from storage
    load_kernel();
    
    // Jump to kernel
    jump_to_kernel();
    
    // Should never reach here
    loop {}
}

/// Run bootloader tasks using async executor
fn run_bootloader_tasks() -> Result<()> {
    let mut executor = AsyncExecutor::new();
    
    // Spawn bootloader tasks
    executor.spawn(async {
        // Initialize storage
        init_storage().await?;
        
        // Verify kernel integrity
        verify_kernel().await?;
        
        // Prepare kernel environment
        prepare_kernel_env().await?;
        
        Ok(())
    })?;
    
    // Run all tasks
    executor.run()
}

/// Async storage initialization
async fn init_storage() -> Result<()> {
    // Simulate storage initialization with delay
    delay(100).await;
    Ok(())
}

/// Async kernel verification
async fn verify_kernel() -> Result<()> {
    // Simulate kernel verification
    for _ in 0..5 {
        delay(20).await;
        yield_now().await;
    }
    Ok(())
}

/// Async kernel environment preparation
async fn prepare_kernel_env() -> Result<()> {
    // Simulate environment preparation
    delay(50).await;
    Ok(())
}

/// Bootloader panic with message
fn boot_panic(_msg: &str) -> ! {
    // In a real implementation, this would log the message
    // For now, just loop forever
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
    unsafe {
        // Kernel entry point address (adjust based on your kernel location)
        let kernel_entry: unsafe extern "C" fn() -> ! = core::mem::transmute(0x08010000 as *const ());
        
        // Disable interrupts
        #[cfg(target_arch = "arm")]
        asm!("cpsid i");
        
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        asm!("csrc mie, 0x888"); // Disable all interrupts
        
        // Jump to kernel
        kernel_entry();
    }
}

/// Panic handler for bootloader
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Simple panic handler for bootloader
    // In a real implementation, this would log to serial or blink LEDs
    loop {}
}



/// Stack top for bootloader
#[link_section = ".stack_top"]
#[used]
static STACK_TOP: u32 = 0x20000000 + 0x4000; // 16KB stack