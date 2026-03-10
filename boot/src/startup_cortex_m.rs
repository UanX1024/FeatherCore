//! Cortex-M startup code

use feathercore_common::arch;
use feathercore_common::arch::util;

/// Interrupt vector table for Cortex-M
#[link_section = ".vector_table"]
#[used]
pub static VECTOR_TABLE: [unsafe extern "C" fn() -> !; 16] = [
    reset_handler,      // Reset
    default_handler,    // NMI
    default_handler,    // HardFault
    default_handler,    // MemManage
    default_handler,    // BusFault
    default_handler,    // UsageFault
    default_handler,    // Reserved
    default_handler,    // Reserved
    default_handler,    // Reserved
    default_handler,    // Reserved
    default_handler,    // SVCall
    default_handler,    // DebugMonitor
    default_handler,    // Reserved
    default_handler,    // PendSV
    default_handler,    // SysTick
    default_handler,    // External Interrupt 0
];

/// Reset handler - entry point for Cortex-M
#[no_mangle]
unsafe extern "C" fn reset_handler() -> ! {
    // Set stack pointer to the top of stack
    util::set_stack_pointer(super::STACK_TOP as usize);
    
    // Initialize architecture
    arch::init();
    
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
    let mut dst = &raw mut _sdata as *mut u32;
    let end = &raw mut _edata as *mut u32;
    
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
    
    let mut dst = &raw mut _sbss as *mut u32;
    let end = &raw mut _ebss as *mut u32;
    
    while dst < end {
        *dst = 0;
        dst = dst.offset(1);
    }
}
