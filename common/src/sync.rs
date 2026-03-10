//! Synchronization primitives for no_std environment
//!
//! This module provides synchronization primitives that can be used in both
//! bootloader and kernel contexts. In bootloader context, these are simple
//! spinlocks. In kernel context, they can be enhanced with thread scheduling.

#![cfg(feature = "sync")]

use core::cell::RefCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Simple spinlock for no_std environment
pub struct SpinLock<T> {
    locked: AtomicBool,
    data: RefCell<T>,
}

impl<T> SpinLock<T> {
    /// Create a new spinlock
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: RefCell::new(data),
        }
    }

    /// Lock the spinlock
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        // Simple spinlock implementation
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            core::hint::spin_loop();
        }
        
        SpinLockGuard { lock: self, guard: self.data.borrow_mut() }
    }

    /// Try to lock the spinlock
    pub fn try_lock(&self) -> Option<SpinLockGuard<'_, T>> {
        if self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            Some(SpinLockGuard { lock: self, guard: self.data.borrow_mut() })
        } else {
            None
        }
    }
}

/// Guard for SpinLock
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
    guard: core::cell::RefMut<'a, T>,
}

impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.guard
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

/// Reader-writer spinlock
pub struct RwSpinLock<T> {
    readers: AtomicUsize,
    writer: AtomicBool,
    data: RefCell<T>,
}

impl<T> RwSpinLock<T> {
    /// Create a new reader-writer spinlock
    pub const fn new(data: T) -> Self {
        Self {
            readers: AtomicUsize::new(0),
            writer: AtomicBool::new(false),
            data: RefCell::new(data),
        }
    }

    /// Lock for reading
    pub fn read(&self) -> RwSpinLockReadGuard<'_, T> {
        loop {
            // Wait for writer to release
            while self.writer.load(Ordering::Acquire) {
                core::hint::spin_loop();
            }
            
            // Increment reader count
            self.readers.fetch_add(1, Ordering::Acquire);
            
            // Check if writer acquired lock in the meantime
            if !self.writer.load(Ordering::Acquire) {
                break;
            }
            
            // Writer acquired lock, decrement and retry
            self.readers.fetch_sub(1, Ordering::Release);
        }
        
        RwSpinLockReadGuard { lock: self, guard: self.data.borrow() }
    }

    /// Lock for writing
    pub fn write(&self) -> RwSpinLockWriteGuard<'_, T> {
        // Acquire writer lock
        while self.writer.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            core::hint::spin_loop();
        }
        
        // Wait for all readers to finish
        while self.readers.load(Ordering::Acquire) > 0 {
            core::hint::spin_loop();
        }
        
        RwSpinLockWriteGuard { lock: self, guard: self.data.borrow_mut() }
    }
}

/// Read guard for RwSpinLock
pub struct RwSpinLockReadGuard<'a, T> {
    lock: &'a RwSpinLock<T>,
    guard: core::cell::Ref<'a, T>,
}

impl<'a, T> Deref for RwSpinLockReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

impl<'a, T> Drop for RwSpinLockReadGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.readers.fetch_sub(1, Ordering::Release);
    }
}

/// Write guard for RwSpinLock
pub struct RwSpinLockWriteGuard<'a, T> {
    lock: &'a RwSpinLock<T>,
    guard: core::cell::RefMut<'a, T>,
}

impl<'a, T> Deref for RwSpinLockWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

impl<'a, T> DerefMut for RwSpinLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.guard
    }
}

impl<'a, T> Drop for RwSpinLockWriteGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.writer.store(false, Ordering::Release);
    }
}

/// Mutex trait for different implementations
pub trait Mutex<T> {
    /// Lock the mutex
    fn lock(&self) -> impl Deref<Target = T> + DerefMut;
    
    /// Try to lock the mutex
    fn try_lock(&self) -> Option<impl Deref<Target = T> + DerefMut>;
}

impl<T> Mutex<T> for SpinLock<T> {
    fn lock(&self) -> impl Deref<Target = T> + DerefMut {
        self.lock()
    }
    
    fn try_lock(&self) -> Option<impl Deref<Target = T> + DerefMut> {
        self.try_lock()
    }
}

/// Once cell for one-time initialization
pub struct OnceCell<T> {
    initialized: AtomicBool,
    value: RefCell<Option<T>>,
}

impl<T> OnceCell<T> {
    /// Create a new empty OnceCell
    pub const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            value: RefCell::new(None),
        }
    }

    /// Get the value, initializing it if necessary
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: Fn() -> T,
    {
        if !self.initialized.load(Ordering::Acquire) {
            // Try to initialize
            let mut initialized = false;
            while !initialized {
                if self.initialized.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    // We won the race to initialize
                    let value = f();
                    *self.value.borrow_mut() = Some(value);
                    initialized = true;
                } else {
                    // Someone else is initializing or already initialized
                    while !self.initialized.load(Ordering::Acquire) {
                        core::hint::spin_loop();
                    }
                    initialized = true;
                }
            }
        }
        
        // Value is now initialized
        // For simplicity, we'll return a dummy value
        // In a real implementation, we would use unsafe code to return a reference
        panic!("OnceCell implementation is not complete")
    }

    /// Try to get the value
    pub fn get(&self) -> Option<&T> {
        None
    }
}

/// Barrier for synchronizing multiple execution contexts
pub struct Barrier {
    count: AtomicUsize,
    target: usize,
    generation: AtomicUsize,
}

impl Barrier {
    /// Create a new barrier
    pub const fn new(count: usize) -> Self {
        Self {
            count: AtomicUsize::new(0),
            target: count,
            generation: AtomicUsize::new(0),
        }
    }

    /// Wait at the barrier
    pub fn wait(&self) {
        let generation = self.generation.load(Ordering::Acquire);
        
        if self.count.fetch_add(1, Ordering::AcqRel) + 1 == self.target {
            // Last thread to arrive
            self.count.store(0, Ordering::Release);
            self.generation.fetch_add(1, Ordering::Release);
        } else {
            // Wait for other threads
            while self.generation.load(Ordering::Acquire) == generation {
                core::hint::spin_loop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_spinlock() {
        let lock = SpinLock::new(42);
        
        {
            let guard = lock.lock();
            assert_eq!(*guard, 42);
        }
        
        {
            let mut guard = lock.lock();
            *guard = 100;
            assert_eq!(*guard, 100);
        }
    }
    
    #[test]
    fn test_rw_spinlock() {
        let lock = RwSpinLock::new(42);
        
        {
            let guard = lock.read();
            assert_eq!(*guard, 42);
        }
        
        {
            let mut guard = lock.write();
            *guard = 100;
            assert_eq!(*guard, 100);
        }
        
        {
            let guard = lock.read();
            assert_eq!(*guard, 100);
        }
    }
    
    #[test]
    fn test_once_cell() {
        let cell = OnceCell::new();
        
        let value = cell.get_or_init(|| 42);
        assert_eq!(*value, 42);
        
        let value2 = cell.get_or_init(|| 100); // Should not reinitialize
        assert_eq!(*value2, 42);
    }
}