//! Utility functions and types

use core::fmt;

/// Align a value up to the given alignment
pub const fn align_up(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

/// Align a value down to the given alignment
pub const fn align_down(value: usize, alignment: usize) -> usize {
    value & !(alignment - 1)
}

/// Check if a value is aligned
pub const fn is_aligned(value: usize, alignment: usize) -> bool {
    value & (alignment - 1) == 0
}

/// Power of two check
pub const fn is_power_of_two(value: usize) -> bool {
    value != 0 && (value & (value - 1)) == 0
}

/// Simple spinlock implementation
pub struct SpinLock<T> {
    data: T,
}

impl<T> SpinLock<T> {
    /// Create a new spinlock
    pub const fn new(data: T) -> Self {
        Self { data }
    }

    /// Lock the spinlock and return a guard
    pub fn lock(&self) -> SpinLockGuard<T> {
        // In a real implementation, this would disable interrupts
        // or use atomic operations
        SpinLockGuard { lock: self }
    }
}

/// Spinlock guard
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<'a, T> core::ops::Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.lock.data
    }
}

impl<'a, T> core::ops::DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: We hold the lock
        unsafe { &mut *(&self.lock.data as *const T as *mut T) }
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        // In a real implementation, this would release the lock
        // and restore interrupt state
    }
}

/// Byte buffer wrapper
pub struct ByteBuffer<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> ByteBuffer<'a> {
    /// Create a new byte buffer
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// Read a byte
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
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Get remaining bytes
    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }
}

impl<'a> fmt::Debug for ByteBuffer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ByteBuffer {{ pos: {}, len: {} }}", self.pos, self.data.len())
    }
}