use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::Ordering;

// 重新导出AtomicBool以便其他模块使用
pub use core::sync::atomic::AtomicBool;

/// Simple spinlock implementation
pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    /// Create a new spinlock
    pub const fn new(data: T) -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }
    
    /// Lock the spinlock
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        while self.locked.swap(true, Ordering::Acquire) {
            // Spin until lock is available
            core::hint::spin_loop();
        }
        SpinLockGuard {
            lock: self,
        }
    }
}

/// Spinlock guard
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

/// Mutex implementation (alias for SpinLock in this simple implementation)
pub struct Mutex<T> {
    inner: SpinLock<T>,
}

impl<T> Mutex<T> {
    /// Create a new mutex
    pub const fn new(data: T) -> Self {
        Mutex {
            inner: SpinLock::new(data),
        }
    }
    
    /// Lock the mutex
    pub fn lock(&self) -> MutexGuard<'_, T> {
        MutexGuard {
            inner: self.inner.lock(),
        }
    }
}

/// Mutex guard
pub struct MutexGuard<'a, T> {
    inner: SpinLockGuard<'a, T>,
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        &*self.inner
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.inner
    }
}



/// Semaphore implementation
pub struct Semaphore {
    count: UnsafeCell<isize>,
    max_count: isize,
    mutex: Mutex<()>,
}

impl Semaphore {
    /// Create a new semaphore
    pub const fn new(initial: isize, max: isize) -> Self {
        Semaphore {
            count: UnsafeCell::new(initial),
            max_count: max,
            mutex: Mutex::new(()),
        }
    }
    
    /// Signal the semaphore
    pub fn signal(&self) {
        let _guard = self.mutex.lock();
        unsafe {
            if *self.count.get() < self.max_count {
                *self.count.get() += 1;
            }
        }
    }
    
    /// Wait for the semaphore
    pub fn wait(&self) {
        loop {
            let _guard = self.mutex.lock();
            unsafe {
                if *self.count.get() > 0 {
                    *self.count.get() -= 1;
                    break;
                }
            }
        }
    }
}

/// Conditional variable implementation
pub struct Condvar {
    _mutex: Mutex<()>,
}

impl Condvar {
    /// Create a new conditional variable
    pub const fn new() -> Self {
        Condvar {
            _mutex: Mutex::new(()),
        }
    }
    
    /// Wait on the conditional variable
    pub fn wait<'a, T>(&self, mutex: &'a Mutex<T>) -> MutexGuard<'a, T> {
        // TODO: Implement proper conditional variable
        mutex.lock()
    }
    
    /// Signal the conditional variable
    pub fn signal(&self) {
        // TODO: Implement proper conditional variable
    }
    
    /// Broadcast the conditional variable
    pub fn broadcast(&self) {
        // TODO: Implement proper conditional variable
    }
}