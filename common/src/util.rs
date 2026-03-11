//! Utility functions and types
//! 实用工具函数和类型

use core::fmt;

/// Align a value up to the given alignment
/// 将值向上对齐到给定的对齐值
/// 
/// # Example
/// ```
/// use feathercore_common::util::align_up;
/// 
/// assert_eq!(align_up(5, 4), 8);
/// assert_eq!(align_up(8, 4), 8);
/// ```
pub const fn align_up(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

/// Align a value down to the given alignment
/// 将值向下对齐到给定的对齐值
/// 
/// # Example
/// ```
/// use feathercore_common::util::align_down;
/// 
/// assert_eq!(align_down(5, 4), 4);
/// assert_eq!(align_down(8, 4), 8);
/// ```
pub const fn align_down(value: usize, alignment: usize) -> usize {
    value & !(alignment - 1)
}

/// Check if a value is aligned
/// 检查值是否对齐
/// 
/// # Example
/// ```
/// use feathercore_common::util::is_aligned;
/// 
/// assert!(is_aligned(8, 4));
/// assert!(!is_aligned(5, 4));
/// ```
pub const fn is_aligned(value: usize, alignment: usize) -> bool {
    value & (alignment - 1) == 0
}

/// Power of two check
/// 检查是否为2的幂
/// 
/// # Example
/// ```
/// use feathercore_common::util::is_power_of_two;
/// 
/// assert!(is_power_of_two(1));
/// assert!(is_power_of_two(2));
/// assert!(is_power_of_two(8));
/// assert!(!is_power_of_two(3));
/// ```
pub const fn is_power_of_two(value: usize) -> bool {
    value != 0 && (value & (value - 1)) == 0
}

/// Simple spinlock implementation
/// 简单的自旋锁实现
pub struct SpinLock<T> {
    data: T,
}

impl<T> SpinLock<T> {
    /// Create a new spinlock
    /// 创建一个新的自旋锁
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::util::SpinLock;
    /// 
    /// let mut lock = SpinLock::new(42);
    /// ```
    pub const fn new(data: T) -> Self {
        Self { data }
    }

    /// Lock the spinlock and return a guard
    /// 锁定自旋锁并返回一个守卫
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::util::SpinLock;
    /// 
    /// let mut lock = SpinLock::new(42);
    /// let guard = lock.lock();
    /// assert_eq!(*guard, 42);
    /// ```
    pub fn lock(&mut self) -> SpinLockGuard<'_, T> {
        // In a real implementation, this would disable interrupts
        // or use atomic operations
        // 在实际实现中，这会禁用中断或使用原子操作
        SpinLockGuard { lock: self }
    }
}

/// Spinlock guard
/// 自旋锁守卫
pub struct SpinLockGuard<'a, T> {
    lock: &'a mut SpinLock<T>,
}

impl<'a, T> core::ops::Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.lock.data
    }
}

impl<'a, T> core::ops::DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lock.data
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        // In a real implementation, this would release the lock
        // and restore interrupt state
        // 在实际实现中，这会释放锁并恢复中断状态
    }
}

/// Byte buffer wrapper
/// 字节缓冲区包装器
pub struct ByteBuffer<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> ByteBuffer<'a> {
    /// Create a new byte buffer
    /// 创建一个新的字节缓冲区
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::util::ByteBuffer;
    /// 
    /// let data = [1, 2, 3, 4, 5];
    /// let mut buffer = ByteBuffer::new(&data);
    /// ```
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// Read a byte
    /// 读取一个字节
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::util::ByteBuffer;
    /// 
    /// let data = [1, 2, 3];
    /// let mut buffer = ByteBuffer::new(&data);
    /// 
    /// assert_eq!(buffer.read_byte(), Some(1));
    /// assert_eq!(buffer.read_byte(), Some(2));
    /// assert_eq!(buffer.read_byte(), Some(3));
    /// assert_eq!(buffer.read_byte(), None);
    /// ```
    pub fn read_byte(&mut self) -> Option<u8> {
        if self.pos < self.data.len() {
            let byte = self.data[self.pos];
            self.pos += 1;
            Some(byte)
        } else {
            None
        }
    }

    /// Read a slice of bytes
    /// 读取字节切片
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::util::ByteBuffer;
    /// 
    /// let data = [1, 2, 3, 4, 5];
    /// let mut buffer = ByteBuffer::new(&data);
    /// 
    /// assert_eq!(buffer.read_slice(2), Some(&[1, 2]));
    /// assert_eq!(buffer.read_slice(2), Some(&[3, 4]));
    /// assert_eq!(buffer.read_slice(2), None); // Not enough bytes
    /// ```
    pub fn read_slice(&mut self, len: usize) -> Option<&'a [u8]> {
        if self.pos + len <= self.data.len() {
            let slice = &self.data[self.pos..self.pos + len];
            self.pos += len;
            Some(slice)
        } else {
            None
        }
    }

    /// Get current position
    /// 获取当前位置
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::util::ByteBuffer;
    /// 
    /// let data = [1, 2, 3, 4, 5];
    /// let mut buffer = ByteBuffer::new(&data);
    /// 
    /// buffer.read_byte();
    /// assert_eq!(buffer.position(), 1);
    /// ```
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Get remaining bytes
    /// 获取剩余字节数
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::util::ByteBuffer;
    /// 
    /// let data = [1, 2, 3, 4, 5];
    /// let mut buffer = ByteBuffer::new(&data);
    /// 
    /// buffer.read_slice(2);
    /// assert_eq!(buffer.remaining(), 3);
    /// ```
    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }
}

impl<'a> fmt::Debug for ByteBuffer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ByteBuffer {{ pos: {}, len: {} }}", self.pos, self.data.len())
    }
}