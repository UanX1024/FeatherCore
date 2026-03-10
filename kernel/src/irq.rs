use crate::sync::Mutex;
use crate::info;

/// Interrupt handler type
pub type InterruptHandler = fn() -> ();

/// Interrupt controller
struct IrqController {
    handlers: [Option<InterruptHandler>; 256],
}

impl IrqController {
    /// Create a new interrupt controller
    pub const fn new() -> Self {
        IrqController {
            handlers: [None; 256],
        }
    }
    
    /// Register an interrupt handler
    pub fn register_handler(&mut self, irq: u8, handler: InterruptHandler) {
        self.handlers[irq as usize] = Some(handler);
    }
    
    /// Handle an interrupt
    pub fn handle_irq(&self, irq: u8) {
        if let Some(handler) = self.handlers[irq as usize] {
            handler();
        } else {
            // Default handler: just acknowledge the interrupt
            info!("Unhandled interrupt: {}", irq);
        }
    }
}

static IRQ_CONTROLLER: Mutex<IrqController> = Mutex::new(IrqController::new());

/// Initialize the interrupt system
pub fn init() {
    // TODO: Set up hardware interrupt controller
    info!("Interrupt system initialized");
}

/// Register an interrupt handler
pub fn register_handler(irq: u8, handler: InterruptHandler) {
    IRQ_CONTROLLER.lock().register_handler(irq, handler);
}

/// Handle an interrupt (called from assembly)
#[no_mangle]
pub extern "C" fn handle_irq(irq: u8) {
    IRQ_CONTROLLER.lock().handle_irq(irq);
}
