#![no_std]

/// RISC-V architecture initialization
/// RISC-V 架构初始化
pub fn init() {
    // RISC-V architecture initialization
    // RISC-V 架构初始化
}

/// Task context structure
/// 任务上下文结构
#[repr(C)]
pub struct TaskContext {
    // Architecture-specific task context
    // 架构特定的任务上下文
    pub stack_pointer: usize,
}

/// Create initial task context
/// 创建初始任务上下文
pub fn create_task_context(stack_pointer: usize) -> TaskContext {
    TaskContext {
        stack_pointer,
    }
}

/// Start the first task
/// 启动第一个任务
pub unsafe fn start_first_task(context: &TaskContext) {
    // RISC-V: Set stack pointer and jump to main
    // RISC-V: 设置栈指针并跳转到 main
    core::arch::asm!(
        "mv sp, {0}",
        "jal main",
        in(reg) context.stack_pointer,
    );
}

/// Switch task context
/// 切换任务上下文
pub unsafe fn switch_context(_from: &TaskContext, _to: &TaskContext) {
    // TODO: Implement context switching for RISC-V
    // TODO: 为 RISC-V 实现上下文切换
}

/// Jump to kernel entry point
/// 跳转到内核入口点
pub unsafe fn jump_to_kernel(vector_table_addr: usize) -> ! {
    // Read reset handler address from vector table (second entry)
    // 从向量表中读取复位处理程序地址（第二个条目）
    let vector_table = vector_table_addr as *const usize;
    let reset_handler_addr = core::ptr::read_volatile(vector_table.add(1));
    
    // Convert address to function pointer using correct type for target
    // 使用目标正确的类型将地址转换为函数指针
    // Use union for safe conversion / 使用 union 进行安全转换
    let addr_usize: usize = reset_handler_addr;
    let kernel_entry = addr_to_fn(addr_usize);
    
    // Disable interrupts
    // 禁用中断
    core::arch::asm!("csrc mie, 0x888"); // Disable all interrupts
    
    // Jump to kernel reset handler
    // 跳转到内核复位处理程序
    kernel_entry();
}

#[allow(transmute_ptr_to_ptr)]
unsafe fn addr_to_fn(addr: usize) -> extern "C" fn() -> ! {
    core::mem::transmute(addr)
}

/// RISC-V startup code
/// RISC-V 启动代码
pub mod startup {
    use core::arch::global_asm;
    
    global_asm!(
        r#"
        .section .vector_table, "aw"
        .globl vector_table
        vector_table:
            .quad _stack_top
            .quad reset_handler
            .quad default_handler
            .quad default_handler
            .quad default_handler
        "#
    );
    
    #[no_mangle]
    extern "C" fn default_handler() -> ! {
        loop {}
    }
    
    #[no_mangle]
    pub unsafe extern "C" fn reset_handler() -> ! {
        // Set stack pointer to the top of stack
        // 将栈指针设置到栈顶
        extern "C" {
            static _stack_top: u32;
        }
        let stack_top = core::ptr::addr_of!(_stack_top) as usize;
        core::arch::asm!("mv sp, {0}", in(reg) stack_top);
        
        // Initialize .data section
        // 初始化 .data 段
        init_data_section();
        
        // Zero-initialize .bss section
        // 零初始化 .bss 段
        zero_bss_section();
        
        // Initialize architecture
        // 初始化架构
        crate::init();
        
        // Call main function
        // 调用主函数
        extern "C" {
            fn boot_main() -> !;
        }
        boot_main();
    }
    
    /// Initialize .data section
    /// 初始化 .data 段
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
    /// 零初始化 .bss 段
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
}