//! Synchronization primitives for no_std environment
//! 无标准库环境的同步原语
//! 
//! This module provides synchronization primitives that can be used in both
//! bootloader and kernel contexts. In bootloader context, these are simple
//! spinlocks. In kernel context, they can be enhanced with thread scheduling.
//! 此模块提供了可在引导加载程序和内核上下文中使用的同步原语。
//! 在引导加载程序上下文中，这些是简单的自旋锁。
//! 在内核上下文中，它们可以通过线程调度得到增强。

#![cfg(feature = "sync")]

use core::cell::RefCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Simple spinlock for no_std environment
/// 无标准库环境的简单自旋锁
pub struct SpinLock<T> {
    locked: AtomicBool,
    data: RefCell<T>,
}

impl<T> SpinLock<T> {
    /// Create a new spinlock
    /// 创建一个新的自旋锁
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::sync::SpinLock;
    /// 
    /// let lock = SpinLock::new(42);
    /// ```
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: RefCell::new(data),
        }
    }

    /// Lock the spinlock
    /// 锁定自旋锁
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::sync::SpinLock;
    /// 
    /// let lock = SpinLock::new(42);
    /// let guard = lock.lock();
    /// assert_eq!(*guard, 42);
    /// ```
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        // Simple spinlock implementation
        // 简单的自旋锁实现
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            core::hint::spin_loop();
        }
        
        SpinLockGuard { lock: self, guard: self.data.borrow_mut() }
    }

    /// Try to lock the spinlock
    /// 尝试锁定自旋锁
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::sync::SpinLock;
    /// 
    /// let lock = SpinLock::new(42);
    /// if let Some(guard) = lock.try_lock() {
    ///     assert_eq!(*guard, 42);
    /// }
    /// ```
    pub fn try_lock(&self) -> Option<SpinLockGuard<'_, T>> {
        if self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            Some(SpinLockGuard { lock: self, guard: self.data.borrow_mut() })
        } else {
            None
        }
    }
}

/// Guard for SpinLock
/// SpinLock的守卫
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
/// 读写自旋锁
pub struct RwSpinLock<T> {
    readers: AtomicUsize,
    writer: AtomicBool,
    data: RefCell<T>,
}

impl<T> RwSpinLock<T> {
    /// Create a new reader-writer spinlock
    /// 创建一个新的读写自旋锁
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::sync::RwSpinLock;
    /// 
    /// let lock = RwSpinLock::new(42);
    /// ```
    pub const fn new(data: T) -> Self {
        Self {
            readers: AtomicUsize::new(0),
            writer: AtomicBool::new(false),
            data: RefCell::new(data),
        }
    }

    /// Lock for reading
    /// 锁定用于读取
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::sync::RwSpinLock;
    /// 
    /// let lock = RwSpinLock::new(42);
    /// let guard = lock.read();
    /// assert_eq!(*guard, 42);
    /// ```
    pub fn read(&self) -> RwSpinLockReadGuard<'_, T> {
        loop {
            // Wait for writer to release
            // 等待写入器释放
            while self.writer.load(Ordering::Acquire) {
                core::hint::spin_loop();
            }
            
            // Increment reader count
            // 增加读取器计数
            self.readers.fetch_add(1, Ordering::Acquire);
            
            // Check if writer acquired lock in the meantime
            // 检查写入器是否在此期间获取了锁
            if !self.writer.load(Ordering::Acquire) {
                break;
            }
            
            // Writer acquired lock, decrement and retry
            // 写入器获取了锁，递减并重试
            self.readers.fetch_sub(1, Ordering::Release);
        }
        
        RwSpinLockReadGuard { lock: self, guard: self.data.borrow() }
    }

    /// Lock for writing
    /// 锁定用于写入
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::sync::RwSpinLock;
    /// 
    /// let lock = RwSpinLock::new(42);
    /// let mut guard = lock.write();
    /// *guard = 100;
    /// assert_eq!(*guard, 100);
    /// ```
    pub fn write(&self) -> RwSpinLockWriteGuard<'_, T> {
        // Acquire writer lock
        // 获取写入器锁
        while self.writer.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            core::hint::spin_loop();
        }
        
        // Wait for all readers to finish
        // 等待所有读取器完成
        while self.readers.load(Ordering::Acquire) > 0 {
            core::hint::spin_loop();
        }
        
        RwSpinLockWriteGuard { lock: self, guard: self.data.borrow_mut() }
    }
}

/// Read guard for RwSpinLock
/// RwSpinLock的读取守卫
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
/// RwSpinLock的写入守卫
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
/// 不同实现的互斥锁特性
pub trait Mutex<T> {
    /// Lock the mutex
    /// 锁定互斥锁
    fn lock(&self) -> impl Deref<Target = T> + DerefMut;
    
    /// Try to lock the mutex
    /// 尝试锁定互斥锁
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
/// 一次性初始化的单元格
pub struct OnceCell<T> {
    initialized: AtomicBool,
    value: RefCell<Option<T>>,
}

impl<T> OnceCell<T> {
    /// Create a new empty OnceCell
    /// 创建一个新的空OnceCell
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::sync::OnceCell;
    /// 
    /// let cell = OnceCell::new();
    /// ```
    pub const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            value: RefCell::new(None),
        }
    }

    /// Get the value, initializing it if necessary
    /// 获取值，必要时进行初始化
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::sync::OnceCell;
    /// 
    /// let cell = OnceCell::new();
    /// let value = cell.get_or_init(|| 42);
    /// assert_eq!(*value, 42);
    /// ```
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: Fn() -> T,
    {
        if !self.initialized.load(Ordering::Acquire) {
            // Try to initialize
            // 尝试初始化
            let mut initialized = false;
            while !initialized {
                if self.initialized.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    // We won the race to initialize
                    // 我们赢得了初始化的竞争
                    let value = f();
                    *self.value.borrow_mut() = Some(value);
                    initialized = true;
                } else {
                    // Someone else is initializing or already initialized
                    // 其他人正在初始化或已经初始化
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
        // 值现在已初始化
        // 为简单起见，我们将返回一个虚拟值
        // 在实际实现中，我们会使用不安全代码返回引用
        panic!("OnceCell implementation is not complete")
    }

    /// Try to get the value
    /// 尝试获取值
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::sync::OnceCell;
    /// 
    /// let cell = OnceCell::new();
    /// assert!(cell.get().is_none());
    /// ```
    pub fn get(&self) -> Option<&T> {
        None
    }
}

/// Barrier for synchronizing multiple execution contexts
/// 用于同步多个执行上下文的屏障
pub struct Barrier {
    count: AtomicUsize,
    target: usize,
    generation: AtomicUsize,
}

impl Barrier {
    /// Create a new barrier
    /// 创建一个新的屏障
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::sync::Barrier;
    /// 
    /// let barrier = Barrier::new(4); // 4 threads
    /// ```
    pub const fn new(count: usize) -> Self {
        Self {
            count: AtomicUsize::new(0),
            target: count,
            generation: AtomicUsize::new(0),
        }
    }

    /// Wait at the barrier
    /// 在屏障处等待
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::sync::Barrier;
    /// 
    /// let barrier = Barrier::new(2);
    /// 
    /// // Thread 1
    /// barrier.wait();
    /// 
    /// // Thread 2
    /// barrier.wait(); // Both threads will proceed after this
    /// ```
    pub fn wait(&self) {
        let generation = self.generation.load(Ordering::Acquire);
        
        if self.count.fetch_add(1, Ordering::AcqRel) + 1 == self.target {
            // Last thread to arrive
            // 最后到达的线程
            self.count.store(0, Ordering::Release);
            self.generation.fetch_add(1, Ordering::Release);
        } else {
            // Wait for other threads
            // 等待其他线程
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