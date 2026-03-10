//! RISC-V startup code

use core::arch::asm;

/// Interrupt vector table for RISC-V
#[link_section = ".vector_table"]
#[used]
pub static VECTOR_TABLE: [unsafe extern "C" fn() -> !; 16] = [
    reset_handler,      // Reset
    default_handler,    // Machine software interrupt
    default_handler,    // Supervisor software interrupt
    default_handler,    // Machine timer interrupt
    default_handler,    // Supervisor timer interrupt
    default_handler,    // Machine external interrupt
    default_handler,    // Supervisor external interrupt
    default_handler,    // Reserved
    default_handler,    // Reserved
    default_handler,    // Reserved
    default_handler,    // Reserved
    default_handler,    // Reserved
    default_handler,    // Reserved
    default_handler,    // Reserved
    default_handler,    // Reserved
    default_handler,    // Reserved
];

/// Reset handler - entry point for RISC-V
#[no_mangle]
unsafe extern "C" fn reset_handler() -> ! {
    // Set stack pointer to the top of stack
    asm!("la sp, {stack_top}", stack_top = sym super::STACK_TOP);
    
    // Initialize .data section
    init_data_section();
    
    // Zero-initialize .bss section
    zero_bss_section();
    
    // Call boot main function
    super::boot_main();
}

/// Default interrupt handler
#[no_mangle]
unsafe extern "C" fn default_handler() -> ! {
    loop {}
}

/// Initialize .data section
unsafe fn init_data_section() {
    extern "C" {
        static mut _sdata: u32;
        static mut _edata: u32;
        static _sidata: u32;
    }
    
    let mut src = &_sidata as *const u32;
    let mut dst = &mut _sdata as *mut u32;
    let end = &mut _edata as *mut u32;
    
    while dst < end {
        *dst = *src;
        dst = dst.offset(1);
        src = src.offset(1);
    }
}

/// Zero-initialize .bss section
unsafe fn zero_bss_section() {
    extern "C" {
        static mut _sbss: u32;
        static mut _ebss: u32;
    }
    
    let mut dst = &mut _sbss as *mut u32;
    let end = &mut _ebss as *mut u32;
    
    while dst < end {
        *dst = 0;
        dst = dst.offset(1);
    }
}
