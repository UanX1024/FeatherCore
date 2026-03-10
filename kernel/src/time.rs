use crate::sync::Mutex;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::info;

/// System tick counter
static TICK_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Timer structure
#[derive(Copy, Clone)]
struct Timer {
    ticks: usize,
    period: usize,
    callback: Option<fn() -> ()>,
    enabled: bool,
}

impl Timer {
    /// Create a new timer
    pub const fn new() -> Self {
        Timer {
            ticks: 0,
            period: 0,
            callback: None,
            enabled: false,
        }
    }
    
    /// Set timer period
    pub fn set_period(&mut self, period: usize) {
        self.period = period;
        self.ticks = 0;
    }
    
    /// Set timer callback
    pub fn set_callback(&mut self, callback: fn() -> ()) {
        self.callback = Some(callback);
    }
    
    /// Enable the timer
    pub fn enable(&mut self) {
        self.enabled = true;
        self.ticks = 0;
    }
    
    /// Disable the timer
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    /// Update timer ticks
    pub fn update(&mut self) -> bool {
        if !self.enabled {
            return false;
        }
        
        self.ticks += 1;
        if self.ticks >= self.period {
            self.ticks = 0;
            if let Some(callback) = self.callback {
                callback();
            }
            return true;
        }
        
        false
    }
}

/// Timer manager
struct TimerManager {
    timers: [Timer; 16],
}

impl TimerManager {
    /// Create a new timer manager
    pub const fn new() -> Self {
        TimerManager {
            timers: [Timer::new(); 16],
        }
    }
    
    /// Allocate a new timer
    pub fn allocate_timer(&mut self) -> Option<usize> {
        for (i, timer) in self.timers.iter().enumerate() {
            if !timer.enabled && timer.callback.is_none() {
                return Some(i);
            }
        }
        None
    }
    
    /// Get a timer by index
    pub fn get_timer(&mut self, index: usize) -> Option<&mut Timer> {
        if index < self.timers.len() {
            Some(&mut self.timers[index])
        } else {
            None
        }
    }
    
    /// Update all timers
    pub fn update_all(&mut self) {
        for timer in &mut self.timers {
            timer.update();
        }
    }
}

static TIMER_MANAGER: Mutex<TimerManager> = Mutex::new(TimerManager::new());

/// Initialize the time system
pub fn init() {
    // TODO: Set up hardware timer
    info!("Time system initialized");
}

/// Get current system ticks
pub fn get_ticks() -> usize {
    TICK_COUNTER.load(Ordering::Relaxed)
}

/// Increment system ticks (called from timer interrupt)
pub fn tick() {
    TICK_COUNTER.fetch_add(1, Ordering::Relaxed);
    TIMER_MANAGER.lock().update_all();
}

/// Allocate a new timer
pub fn allocate_timer() -> Option<usize> {
    TIMER_MANAGER.lock().allocate_timer()
}

/// Set timer period
pub fn set_timer_period(timer_id: usize, period: usize) -> bool {
    if let Some(timer) = TIMER_MANAGER.lock().get_timer(timer_id) {
        timer.set_period(period);
        true
    } else {
        false
    }
}

/// Set timer callback
pub fn set_timer_callback(timer_id: usize, callback: fn() -> ()) -> bool {
    if let Some(timer) = TIMER_MANAGER.lock().get_timer(timer_id) {
        timer.set_callback(callback);
        true
    } else {
        false
    }
}

/// Enable timer
pub fn enable_timer(timer_id: usize) -> bool {
    if let Some(timer) = TIMER_MANAGER.lock().get_timer(timer_id) {
        timer.enable();
        true
    } else {
        false
    }
}

/// Disable timer
pub fn disable_timer(timer_id: usize) -> bool {
    if let Some(timer) = TIMER_MANAGER.lock().get_timer(timer_id) {
        timer.disable();
        true
    } else {
        false
    }
}
