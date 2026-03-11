#![no_std]

use core::fmt;

/// The 32-bit value the stack is painted with before the program runs.
/// 程序运行前用于填充栈的32位值。
#[cfg(feature = "paint-stack")]
pub const STACK_PAINT_VALUE: u32 = 0xcccc_cccc;

#[cfg(cortex_m)]
use core::arch::global_asm;

/// Parse cfg attributes inside a global_asm call.
/// 在 global_asm 调用中解析 cfg 属性。
#[cfg(cortex_m)]
macro_rules! cfg_global_asm {
    {@inner, [$($x:tt)*], } => {
        global_asm!{$($x)*}
    };
    {@inner, [$($x:tt)*], #[cfg($meta:meta)] $asm:literal, $($rest:tt)*} => {
        #[cfg($meta)]
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
        #[cfg(not($meta))]
        cfg_global_asm!{@inner, [$($x)*], $($rest)*}
    };
    {@inner, [$($x:tt)*], $asm:literal, $($rest:tt)*} => {
        cfg_global_asm!{@inner, [$($x)* $asm,], $($rest)*}
    };
    {$($asms:tt)*} => {
        cfg_global_asm!{@inner, [], $($asms)*}
    };
}

// This reset vector is the initial entry point after a system reset.
// Calls an optional user-provided __pre_init and then initialises RAM.
// If the target has an FPU, it is enabled.
// Finally jumps to the user main function.
// 此复位向量是系统复位后的初始入口点。
// 调用可选的用户提供的 __pre_init，然后初始化 RAM。
// 如果目标有 FPU，则启用它。
// 最后跳转到用户主函数。
#[cfg(cortex_m)]
cfg_global_asm! {
    ".cfi_sections .debug_frame
     .section .Reset, \"ax\"
     .global Reset
     .type Reset,%function
     .thumb_func",
    ".cfi_startproc
     Reset:",

    // If enabled, initialise the SP. This is normally initialised by the CPU itself or by a
    // bootloader, but some debuggers fail to set it when resetting the target, leading to
    // stack corruptions.
    // 如果启用，初始化 SP。这通常由 CPU 本身或引导加载程序初始化，
    // 但一些调试器在复位目标时未能设置它，导致栈损坏。
    #[cfg(feature = "set-sp")]
    "ldr r0, =_stack_start
     msr msp, r0",

    // If enabled, initialise VTOR to the start of the vector table. This is normally initialised
    // by a bootloader when the non-reset value is required, but some bootloaders do not set it,
    // leading to frustrating issues where everything seems to work but interrupts are never
    // handled. The VTOR register is optional on ARMv6-M, but when not present is RAZ,WI and
    // therefore safe to write to.
    // 如果启用，将 VTOR 初始化为向量表的开始。这通常在需要非复位值时由引导加载程序初始化，
    // 但一些引导加载程序不设置它，导致一切似乎正常工作但中断从未被处理的令人沮丧的问题。
    // VTOR 寄存器在 ARMv6-M 上是可选的，但不存在时是 RAZ,WI，因此可以安全写入。
    #[cfg(feature = "set-vtor")]
    "ldr r0, =0xe000ed08
     ldr r1, =__vector_table
     str r1, [r0]",

    // If enabled, set the Main Stack Pointer Limit (MSPLIM) to the end of the stack.
    // This feature is only available on ARMv8-M Mainline, where it helps enforce stack limits
    // by defining the lowest valid stack address.
    // 如果启用，将主栈指针限制 (MSPLIM) 设置为栈的末尾。
    // 此功能仅在 ARMv8-M Mainline 上可用，通过定义最低有效栈地址来帮助强制执行栈限制。
    #[cfg(all(armv8m_main, feature = "set-msplim"))]
    "ldr r0, =_stack_end
     msr MSPLIM, r0",

    // Run user pre-init code which must be executed immediately after startup, before the
    // potentially time-consuming memory initialisation takes place.
    // Example use cases include disabling default watchdogs or enabling RAM.
    // 运行用户预初始化代码，这些代码必须在启动后立即执行，在可能耗时的内存初始化之前。
    // 示例用例包括禁用默认看门狗或启用 RAM。
    "bl __pre_init",

    // If enabled, initialize RAM with zeros. This is not usually required, but might be necessary
    // to properly initialize checksum-based memory integrity measures on safety-critical hardware.
    // 如果启用，用零初始化 RAM。这通常不是必需的，但在安全关键硬件上可能需要正确初始化基于校验和的内存完整性措施。
    #[cfg(feature = "zero-init-ram")]
    "ldr r0, =_ram_start
     ldr r1, =_ram_end
     movs r2, #0
     0:
     cmp r1, r0
     beq 1f
     stm r0!, {{r2}}
     b 0b
     1:",

    // Initialise .bss memory. `__sbss` and `__ebss` come from the linker script.
    // 初始化 .bss 内存。`__sbss` 和 `__ebss` 来自链接脚本。
    #[cfg(not(feature = "zero-init-ram"))]
    "ldr r0, =__sbss
     ldr r1, =__ebss
     movs r2, #0
     0:
     cmp r1, r0
     beq 1f
     stm r0!, {{r2}}
     b 0b
     1:",

    // If enabled, paint stack/heap RAM with 0xcccccccc.
    // `_stack_end` and `_stack_start` come from the linker script.
    // 如果启用，用 0xcccccccc 填充栈/堆 RAM。
    // `_stack_end` 和 `_stack_start` 来自链接脚本。
    #[cfg(feature = "paint-stack")]
    "ldr r0, =_stack_end
     ldr r1, =_stack_start
     ldr r2, =0xcccccccc // This must match STACK_PAINT_VALUE
     0:
     cmp r1, r0
     beq 1f
     stm r0!, {{r2}}
     b 0b
     1:",

    // Initialise .data memory. `__sdata`, `__sidata`, and `__edata` come from the linker script.
    // 初始化 .data 内存。`__sdata`、`__sidata` 和 `__edata` 来自链接脚本。
    #[cfg(not(feature = "skip-data-copy"))]
    "ldr r0, =__sdata
     ldr r1, =__edata
     ldr r2, =__sidata
     0:
     cmp r1, r0
     beq 1f
     ldm r2!, {{r3}}
     stm r0!, {{r3}}
     b 0b
     1:",

    // Potentially enable an FPU.
    // SCB.CPACR is 0xE000_ED88.
    // We enable access to CP10 and CP11 from priviliged and unprivileged mode.
    // 可能启用 FPU。
    // SCB.CPACR 是 0xE000_ED88。
    // 我们从特权和非特权模式启用对 CP10 和 CP11 的访问。
    #[cfg(has_fpu)]
    "ldr r0, =0xE000ED88
     ldr r1, =(0b1111 << 20)
     ldr r2, [r0]
     orr r2, r2, r1
     str r2, [r0]
     dsb
     isb",

    // Jump to user main function.
    // `bl` is used for the extended range, but the user main function should not return,
    // so trap on any unexpected return.
    // 跳转到用户主函数。
    // `bl` 用于扩展范围，但用户主函数不应返回，因此在任何意外返回时陷阱。
    "bl main
     udf #0",

    ".cfi_endproc
     .size Reset, . - Reset",
}

/// Registers stacked (pushed onto the stack) during an exception.
/// 异常期间堆叠（压入栈）的寄存器。
#[derive(Clone, Copy)]
#[repr(C)]
#[allow(dead_code)]
pub struct ExceptionFrame {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: u32,
}

impl ExceptionFrame {
    /// Returns the value of (general purpose) register 0.
    /// 返回（通用）寄存器 0 的值。
    #[inline(always)]
    pub fn r0(&self) -> u32 {
        self.r0
    }

    /// Returns the value of (general purpose) register 1.
    /// 返回（通用）寄存器 1 的值。
    #[inline(always)]
    pub fn r1(&self) -> u32 {
        self.r1
    }

    /// Returns the value of (general purpose) register 2.
    /// 返回（通用）寄存器 2 的值。
    #[inline(always)]
    pub fn r2(&self) -> u32 {
        self.r2
    }

    /// Returns the value of (general purpose) register 3.
    /// 返回（通用）寄存器 3 的值。
    #[inline(always)]
    pub fn r3(&self) -> u32 {
        self.r3
    }

    /// Returns the value of (general purpose) register 12.
    /// 返回（通用）寄存器 12 的值。
    #[inline(always)]
    pub fn r12(&self) -> u32 {
        self.r12
    }

    /// Returns the value of the Link Register.
    /// 返回链接寄存器的值。
    #[inline(always)]
    pub fn lr(&self) -> u32 {
        self.lr
    }

    /// Returns the value of the Program Counter.
    /// 返回程序计数器的值。
    #[inline(always)]
    pub fn pc(&self) -> u32 {
        self.pc
    }

    /// Returns the value of the Program Status Register.
    /// 返回程序状态寄存器的值。
    #[inline(always)]
    pub fn xpsr(&self) -> u32 {
        self.xpsr
    }

    /// Sets the stacked value of (general purpose) register 0.
    /// 设置（通用）寄存器 0 的堆叠值。
    ///
    /// # Safety
    /// 安全
    ///
    /// This affects the `r0` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    /// 这会影响被抢占代码的 `r0` 寄存器，该代码不得依赖于它恢复到之前的值。
    #[inline(always)]
    pub unsafe fn set_r0(&mut self, value: u32) {
        self.r0 = value;
    }

    /// Sets the stacked value of (general purpose) register 1.
    /// 设置（通用）寄存器 1 的堆叠值。
    ///
    /// # Safety
    /// 安全
    ///
    /// This affects the `r1` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    /// 这会影响被抢占代码的 `r1` 寄存器，该代码不得依赖于它恢复到之前的值。
    #[inline(always)]
    pub unsafe fn set_r1(&mut self, value: u32) {
        self.r1 = value;
    }

    /// Sets the stacked value of (general purpose) register 2.
    /// 设置（通用）寄存器 2 的堆叠值。
    ///
    /// # Safety
    /// 安全
    ///
    /// This affects the `r2` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    /// 这会影响被抢占代码的 `r2` 寄存器，该代码不得依赖于它恢复到之前的值。
    #[inline(always)]
    pub unsafe fn set_r2(&mut self, value: u32) {
        self.r2 = value;
    }

    /// Sets the stacked value of (general purpose) register 3.
    /// 设置（通用）寄存器 3 的堆叠值。
    ///
    /// # Safety
    /// 安全
    ///
    /// This affects the `r3` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    /// 这会影响被抢占代码的 `r3` 寄存器，该代码不得依赖于它恢复到之前的值。
    #[inline(always)]
    pub unsafe fn set_r3(&mut self, value: u32) {
        self.r3 = value;
    }

    /// Sets the stacked value of (general purpose) register 12.
    /// 设置（通用）寄存器 12 的堆叠值。
    ///
    /// # Safety
    /// 安全
    ///
    /// This affects the `r12` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    /// 这会影响被抢占代码的 `r12` 寄存器，该代码不得依赖于它恢复到之前的值。
    #[inline(always)]
    pub unsafe fn set_r12(&mut self, value: u32) {
        self.r12 = value;
    }

    /// Sets the stacked value of the Link Register.
    /// 设置链接寄存器的堆叠值。
    ///
    /// # Safety
    /// 安全
    ///
    /// This affects the `lr` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    /// 这会影响被抢占代码的 `lr` 寄存器，该代码不得依赖于它恢复到之前的值。
    #[inline(always)]
    pub unsafe fn set_lr(&mut self, value: u32) {
        self.lr = value;
    }

    /// Sets the stacked value of the Program Counter.
    /// 设置程序计数器的堆叠值。
    ///
    /// # Safety
    /// 安全
    ///
    /// This affects the `pc` register of the preempted code, which must not rely on it getting
    /// restored to its previous value.
    /// 这会影响被抢占代码的 `pc` 寄存器，该代码不得依赖于它恢复到之前的值。
    #[inline(always)]
    pub unsafe fn set_pc(&mut self, value: u32) {
        self.pc = value;
    }

    /// Sets the stacked value of the Program Status Register.
    /// 设置程序状态寄存器的堆叠值。
    ///
    /// # Safety
    /// 安全
    ///
    /// This affects the `xPSR` registers (`IPSR`, `APSR`, and `EPSR`) of the preempted code, which
    /// must not rely on them getting restored to their previous value.
    /// 这会影响被抢占代码的 `xPSR` 寄存器（`IPSR`、`APSR` 和 `EPSR`），这些寄存器不得依赖于它们恢复到之前的值。
    #[inline(always)]
    pub unsafe fn set_xpsr(&mut self, value: u32) {
        self.xpsr = value;
    }
}

impl fmt::Debug for ExceptionFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Hex(u32);
        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "0x{:08x}", self.0)
            }
        }
        f.debug_struct("ExceptionFrame")
            .field("r0", &Hex(self.r0))
            .field("r1", &Hex(self.r1))
            .field("r2", &Hex(self.r2))
            .field("r3", &Hex(self.r3))
            .field("r12", &Hex(self.r12))
            .field("lr", &Hex(self.lr))
            .field("pc", &Hex(self.pc))
            .field("xpsr", &Hex(self.xpsr))
            .finish()
    }
}

/// ARM architecture initialization
/// ARM 架构初始化
pub fn init() {
    // Architecture initialization is done via feature flags
    // 架构初始化通过特性标志完成
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
    #[cfg(any(feature = "armv6-m", feature = "armv7-m", feature = "armv7-em", feature = "armv8-m-base", feature = "armv8-m-main"))]
    {
        // Cortex-M: Set stack pointer and jump to main
        // Cortex-M: 设置栈指针并跳转到 main
        core::arch::asm!(
            "mov sp, {0}",
            "b main",
            in(reg) context.stack_pointer,
        );
    }
    
    #[cfg(any(feature = "armv7-a", feature = "armv8-a", feature = "armv9-a"))]
    {
        // Cortex-A: Set stack pointer and jump to main
        // Cortex-A: 设置栈指针并跳转到 main
        core::arch::asm!(
            "mov sp, {0}",
            "b main",
            in(reg) context.stack_pointer,
        );
    }
}

/// Switch task context
/// 切换任务上下文
pub unsafe fn switch_context(_from: &TaskContext, _to: &TaskContext) {
    // TODO: Implement context switching for different ARM architectures
    // TODO: 为不同的 ARM 架构实现上下文切换
}

/// Jump to kernel entry point
/// 跳转到内核入口点
pub unsafe fn jump_to_kernel(vector_table_addr: usize) -> ! {
    // Read reset handler address from vector table (second entry)
    // 从向量表中读取复位处理程序地址（第二个条目）
    let vector_table = vector_table_addr as *const u32;
    let reset_handler_addr = *vector_table.add(1); // Offset 4 bytes
    
    // Convert to function pointer
    // 转换为函数指针
    let kernel_entry: unsafe extern "C" fn() -> ! = core::mem::transmute(reset_handler_addr);
    
    // Disable interrupts
    // 禁用中断
    util::disable_interrupts();
    
    // Jump to kernel reset handler
    // 跳转到内核复位处理程序
    kernel_entry();
}

/// Returns a pointer to the start of the heap
/// 返回堆的开始指针
///
/// The returned pointer is guaranteed to be 4-byte aligned.
/// 返回的指针保证是 4 字节对齐的。
#[inline]
pub fn heap_start() -> *mut u32 {
    extern "C" {
        static mut __sheap: u32;
    }

    #[allow(unused_unsafe)]
    unsafe {
        core::ptr::addr_of_mut!(__sheap)
    }
}

// Entry point is Reset.
// 入口点是 Reset。
#[doc(hidden)]
#[cfg_attr(cortex_m, link_section = ".vector_table.reset_vector")]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

// Exceptions
// 异常
extern "C" {
    fn Reset() -> !;
}

/// Entry point macro
/// 入口点宏
///
/// This macro marks a function as the entry point of the program.
/// The function must have signature `[unsafe] fn() -> !`
/// 此宏将函数标记为程序的入口点。
/// 函数必须具有签名 `[unsafe] fn() -> !`
#[macro_export]
macro_rules! entry {
    ($($item:item)*) => {
        $(#[$item])*
        #[export_name = "main"]
        pub unsafe extern "C" fn main() -> ! {
            $($item)*
        }
    };
}

/// Exception handler macro
/// 异常处理程序宏
///
/// This macro marks a function as an exception handler.
/// The function must have signature `[unsafe] fn() [-> !]`
/// 此宏将函数标记为异常处理程序。
/// 函数必须具有签名 `[unsafe] fn() [-> !]`
#[macro_export]
macro_rules! exception {
    ($($item:item)*) => {
        $(#[$item])*
        pub unsafe extern "C" fn $($item)*
    };
}

/// Pre-init function macro
/// 预初始化函数宏
///
/// This macro marks a function as a pre-init function that runs before RAM initialization.
/// The function must have signature `unsafe fn()`
/// 此宏将函数标记为在 RAM 初始化之前运行的预初始化函数。
/// 函数必须具有签名 `unsafe fn()`
#[macro_export]
macro_rules! pre_init {
    ($($item:item)*) => {
        $(#[$item])*
        #[export_name = "__pre_init"]
        pub unsafe fn $($item)*
    };
}

/// Interrupt handler macro
/// 中断处理程序宏
///
/// This macro marks a function as an interrupt handler.
/// The function must have signature `[unsafe] fn() [-> !]`
/// 此宏将函数标记为中断处理程序。
/// 函数必须具有签名 `[unsafe] fn() [-> !]`
#[cfg(feature = "device")]
#[macro_export]
macro_rules! interrupt {
    ($($item:item)*) => {
        $(#[$item])*
        pub unsafe extern "C" fn $($item)*
    };
}

/// Architecture-specific utilities
/// 架构特定的实用程序
pub mod util {
    /// Enable interrupts
    /// 启用中断
    pub unsafe fn enable_interrupts() {
        #[cfg(any(feature = "armv6-m", feature = "armv7-m", feature = "armv7-em", feature = "armv8-m-base", feature = "armv8-m-main"))]
        { 
            // Cortex-M: Enable interrupts
            // Cortex-M: 启用中断
            core::arch::asm!("cpsie i");
        }
        
        #[cfg(any(feature = "armv7-a", feature = "armv8-a", feature = "armv9-a"))]
        { 
            // Cortex-A: Enable interrupts
            // Cortex-A: 启用中断
            let mut cpsr: u32;
            core::arch::asm!("mrs {}, cpsr", out(reg) cpsr);
            cpsr &= !0x80; // Clear I bit
            core::arch::asm!("msr cpsr_c, {}", in(reg) cpsr);
        }
    }
    
    /// Disable interrupts
    /// 禁用中断
    pub unsafe fn disable_interrupts() {
        #[cfg(any(feature = "armv6-m", feature = "armv7-m", feature = "armv7-em", feature = "armv8-m-base", feature = "armv8-m-main"))]
        { 
            // Cortex-M: Disable interrupts
            // Cortex-M: 禁用中断
            core::arch::asm!("cpsid i");
        }
        
        #[cfg(any(feature = "armv7-a", feature = "armv8-a", feature = "armv9-a"))]
        { 
            // Cortex-A: Disable interrupts
            // Cortex-A: 禁用中断
            let mut cpsr: u32;
            core::arch::asm!("mrs {}, cpsr", out(reg) cpsr);
            cpsr |= 0x80; // Set I bit
            core::arch::asm!("msr cpsr_c, {}", in(reg) cpsr);
        }
    }
    
    /// Get stack pointer
    /// 获取栈指针
    pub unsafe fn get_stack_pointer() -> usize {
        let sp: usize;
        core::arch::asm!("mov {}, sp", out(reg) sp);
        sp
    }
    
    /// Set stack pointer
    /// 设置栈指针
    pub unsafe fn set_stack_pointer(sp: usize) {
        core::arch::asm!("mov sp, {}", in(reg) sp);
    }
}

/// Exception types
/// 异常类型
#[doc(hidden)]
pub enum Exception {
    NonMaskableInt,

    // Not overridable
    // 不可覆盖
    // HardFault,
    #[cfg(not(armv6m))]
    MemoryManagement,

    #[cfg(not(armv6m))]
    BusFault,

    #[cfg(not(armv6m))]
    UsageFault,

    #[cfg(armv8m)]
    SecureFault,

    SVCall,

    #[cfg(not(armv6m))]
    DebugMonitor,

    PendSV,

    SysTick,
}

#[doc(hidden)]
pub use self::Exception as exception;

/// Vector union for interrupt handlers
/// 中断处理程序的向量联合
#[doc(hidden)]
#[repr(C)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

/// Default pre-init function
/// 默认预初始化函数
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn __pre_init() {}

/// Default HardFault handler
/// 默认 HardFault 处理程序
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn HardFault_() {
    #[allow(clippy::empty_loop)]
    loop {}
}

/// Default exception handler
/// 默认异常处理程序
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultHandler_() {
    #[allow(clippy::empty_loop)]
    loop {}
}

// We export this static with an informative name so that if an application attempts to link
// two copies of the ARM runtime together, linking will fail.
// 我们使用信息性名称导出此静态变量，以便如果应用程序尝试链接
// 两个 ARM 运行时副本，链接将失败。
#[export_name = "error: feather_core::arch::arm appears more than once in the dependency graph"]
#[doc(hidden)]
pub static __ONCE__: () = ();

/// Default pre-init function
/// 默认预初始化函数
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultPreInit() {}

/// Default exception handler
/// 默认异常处理程序
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DefaultHandler() {
    #[allow(clippy::empty_loop)]
    loop {}
}

/// Vector table
/// 向量表
#[cfg(cortex_m)]
#[doc(hidden)]
#[link_section = ".vector_table"]
#[no_mangle]
pub static __vector_table: [u32; 16] = {
    extern "C" {
        static _stack_start: u32;
    }
    
    [
        &_stack_start as *const u32 as u32, // Stack pointer
        Reset as *const () as u32,          // Reset
        NonMaskableInt as *const () as u32, // NMI
        HardFault as *const () as u32,      // HardFault
        MemoryManagement as *const () as u32, // MemManage
        BusFault as *const () as u32,        // BusFault
        UsageFault as *const () as u32,      // UsageFault
        0,                                  // Reserved
        0,                                  // Reserved
        0,                                  // Reserved
        0,                                  // Reserved
        SVCall as *const () as u32,          // SVCall
        DebugMonitor as *const () as u32,    // DebugMonitor
        0,                                  // Reserved
        PendSV as *const () as u32,          // PendSV
        SysTick as *const () as u32,         // SysTick
    ]
};

// Default exception handlers
// 默认异常处理程序
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn NonMaskableInt() {
    DefaultHandler();
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn HardFault() {
    HardFault_();
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn MemoryManagement() {
    DefaultHandler();
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn BusFault() {
    DefaultHandler();
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn UsageFault() {
    DefaultHandler();
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn SecureFault() {
    DefaultHandler();
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn SVCall() {
    DefaultHandler();
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn DebugMonitor() {
    DefaultHandler();
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn PendSV() {
    DefaultHandler();
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn SysTick() {
    DefaultHandler();
}

// If we are not targeting a specific device we bind all the potential device specific interrupts
// to the default handler
// 如果我们没有针对特定设备，我们将所有潜在的设备特定中断绑定到默认处理程序
#[cfg(all(any(not(feature = "device"), test), not(armv6m), not(armv8m_main)))]
#[doc(hidden)]
#[cfg_attr(cortex_m, link_section = ".vector_table.interrupts")]
#[no_mangle]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 240] = [DefaultHandler; 240];

// ARMv8-M Mainline can have up to 480 device specific interrupts
// ARMv8-M Mainline 最多可以有 480 个设备特定中断
#[cfg(all(not(feature = "device"), armv8m_main))]
#[doc(hidden)]
#[cfg_attr(cortex_m, link_section = ".vector_table.interrupts")]
#[no_mangle]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 480] = [DefaultHandler; 480];

// ARMv6-M can only have a maximum of 32 device specific interrupts
// ARMv6-M 最多只能有 32 个设备特定中断
#[cfg(all(not(feature = "device"), armv6m))]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 32] = [DefaultHandler; 32];