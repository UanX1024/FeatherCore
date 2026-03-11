//! Memory management utilities for no_std environment
//! 无标准库环境的内存管理工具
//! 
//! This module provides basic memory management functionality that can be used
//! in both bootloader and kernel contexts.
//! 此模块提供了可在引导加载程序和内核上下文中使用的基本内存管理功能。

#![cfg(feature = "mm")]

use core::alloc::Layout;
use core::ptr::NonNull;

use crate::Error;
use crate::Result;

/// Simple bump allocator for bootloader and early kernel initialization
/// 用于引导加载程序和早期内核初始化的简单线性分配器
pub struct BumpAllocator {
    start: usize,
    end: usize,
    current: usize,
}

impl BumpAllocator {
    /// Create a new bump allocator with the given memory region
    /// 使用给定的内存区域创建一个新的线性分配器
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::BumpAllocator;
    /// 
    /// let mut buffer = [0u8; 1024];
    /// let mut allocator = BumpAllocator::new(buffer.as_ptr() as usize, buffer.len());
    /// ```
    pub const fn new(start: usize, size: usize) -> Self {
        Self {
            start,
            end: start + size,
            current: start,
        }
    }

    /// Allocate memory with the given layout
    /// 使用给定的布局分配内存
    /// 
    /// # Example
    /// ```
    /// use core::alloc::Layout;
    /// use feathercore_common::mm::BumpAllocator;
    /// 
    /// let mut buffer = [0u8; 1024];
    /// let mut allocator = BumpAllocator::new(buffer.as_ptr() as usize, buffer.len());
    /// 
    /// let layout = Layout::from_size_align(64, 8).unwrap();
    /// let ptr = allocator.allocate(layout).unwrap();
    /// ```
    pub fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>> {
        let aligned_addr = crate::util::align_up(self.current, layout.align());
        let new_current = aligned_addr + layout.size();

        if new_current > self.end {
            return Err(Error::OutOfMemory);
        }

        self.current = new_current;

        // Use safe NonNull::new instead of unsafe new_unchecked
        // 使用安全的NonNull::new而不是不安全的new_unchecked
        NonNull::new(aligned_addr as *mut u8)
            .ok_or(Error::OutOfMemory)
    }

    /// Reset the allocator to its initial state
    /// 将分配器重置到初始状态
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::BumpAllocator;
    /// 
    /// let mut buffer = [0u8; 1024];
    /// let mut allocator = BumpAllocator::new(buffer.as_ptr() as usize, buffer.len());
    /// 
    /// // Allocate some memory
    /// // ...
    /// 
    /// // Reset the allocator
    /// allocator.reset();
    /// ```
    pub fn reset(&mut self) {
        self.current = self.start;
    }

    /// Get the amount of memory used
    /// 获取已使用的内存量
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::BumpAllocator;
    /// 
    /// let mut buffer = [0u8; 1024];
    /// let mut allocator = BumpAllocator::new(buffer.as_ptr() as usize, buffer.len());
    /// 
    /// // Allocate some memory
    /// // ...
    /// 
    /// println!("Used memory: {} bytes", allocator.used());
    /// ```
    pub fn used(&self) -> usize {
        self.current - self.start
    }

    /// Get the amount of memory available
    /// 获取可用的内存量
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::BumpAllocator;
    /// 
    /// let mut buffer = [0u8; 1024];
    /// let mut allocator = BumpAllocator::new(buffer.as_ptr() as usize, buffer.len());
    /// 
    /// // Allocate some memory
    /// // ...
    /// 
    /// println!("Available memory: {} bytes", allocator.available());
    /// ```
    pub fn available(&self) -> usize {
        self.end - self.current
    }

    /// Get the total memory size
    /// 获取总内存大小
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::BumpAllocator;
    /// 
    /// let mut buffer = [0u8; 1024];
    /// let mut allocator = BumpAllocator::new(buffer.as_ptr() as usize, buffer.len());
    /// 
    /// println!("Total memory: {} bytes", allocator.total());
    /// ```
    pub fn total(&self) -> usize {
        self.end - self.start
    }
}

/// Memory region descriptor
/// 内存区域描述符
#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    /// Start address of the region
    /// 区域的起始地址
    pub start: usize,
    /// Size of the region in bytes
    /// 区域的大小（字节）
    pub size: usize,
    /// Memory type
    /// 内存类型
    pub memory_type: MemoryType,
    /// Memory attributes
    /// 内存属性
    pub attributes: MemoryAttributes,
}

/// Memory type
/// 内存类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    /// RAM (read-write executable)
    /// RAM（可读写可执行）
    Ram,
    /// ROM (read-only executable)
    /// ROM（只读可执行）
    Rom,
    /// Device memory (read-write, non-executable)
    /// 设备内存（可读写，不可执行）
    Device,
    /// Reserved memory (not usable)
    /// 保留内存（不可用）
    Reserved,
}

/// Memory attributes
/// 内存属性
#[derive(Debug, Clone, Copy, Default)]
pub struct MemoryAttributes {
    /// Readable
    /// 可读
    pub readable: bool,
    /// Writable
    /// 可写
    pub writable: bool,
    /// Executable
    /// 可执行
    pub executable: bool,
    /// Cacheable
    /// 可缓存
    pub cacheable: bool,
    /// Shareable
    /// 可共享
    pub shareable: bool,
}

impl MemoryRegion {
    /// Create a new memory region
    /// 创建一个新的内存区域
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::{MemoryRegion, MemoryType, MemoryAttributes};
    /// 
    /// let region = MemoryRegion::new(
    ///     0x1000,
    ///     0x1000,
    ///     MemoryType::Ram,
    ///     MemoryAttributes {
    ///         readable: true,
    ///         writable: true,
    ///         executable: true,
    ///         ..Default::default()
    ///     },
    /// );
    /// ```
    pub const fn new(
        start: usize,
        size: usize,
        memory_type: MemoryType,
        attributes: MemoryAttributes,
    ) -> Self {
        Self {
            start,
            size,
            memory_type,
            attributes,
        }
    }

    /// Check if an address is within this region
    /// 检查地址是否在此区域内
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::{MemoryRegion, MemoryType, MemoryAttributes};
    /// 
    /// let region = MemoryRegion::new(
    ///     0x1000,
    ///     0x1000,
    ///     MemoryType::Ram,
    ///     MemoryAttributes::default(),
    /// );
    /// 
    /// assert!(region.contains(0x1000));
    /// assert!(region.contains(0x1FFF));
    /// assert!(!region.contains(0x2000));
    /// ```
    pub fn contains(&self, addr: usize) -> bool {
        addr >= self.start && addr < self.start + self.size
    }

    /// Get the end address (exclusive)
    /// 获取结束地址（不包含）
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::{MemoryRegion, MemoryType, MemoryAttributes};
    /// 
    /// let region = MemoryRegion::new(
    ///     0x1000,
    ///     0x1000,
    ///     MemoryType::Ram,
    ///     MemoryAttributes::default(),
    /// );
    /// 
    /// assert_eq!(region.end(), 0x2000);
    /// ```
    pub fn end(&self) -> usize {
        self.start + self.size
    }

    /// Check if this region overlaps with another region
    /// 检查此区域是否与另一个区域重叠
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::{MemoryRegion, MemoryType, MemoryAttributes};
    /// 
    /// let region1 = MemoryRegion::new(
    ///     0x1000,
    ///     0x1000,
    ///     MemoryType::Ram,
    ///     MemoryAttributes::default(),
    /// );
    /// 
    /// let region2 = MemoryRegion::new(
    ///     0x1500,
    ///     0x1000,
    ///     MemoryType::Ram,
    ///     MemoryAttributes::default(),
    /// );
    /// 
    /// assert!(region1.overlaps(&region2));
    /// ```
    pub fn overlaps(&self, other: &MemoryRegion) -> bool {
        self.start < other.end() && other.start < self.end()
    }
}

/// Memory map for the system
/// 系统内存映射
pub struct MemoryMap {
    regions: [Option<MemoryRegion>; 16],
    region_count: usize,
}

impl MemoryMap {
    /// Create a new empty memory map
    /// 创建一个新的空内存映射
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::MemoryMap;
    /// 
    /// let mut memory_map = MemoryMap::new();
    /// ```
    pub fn new() -> Self {
        Self {
            regions: [None; 16],
            region_count: 0,
        }
    }

    /// Add a memory region to the map
    /// 向映射添加内存区域
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::{MemoryMap, MemoryRegion, MemoryType, MemoryAttributes};
    /// 
    /// let mut memory_map = MemoryMap::new();
    /// 
    /// let region = MemoryRegion::new(
    ///     0x1000,
    ///     0x1000,
    ///     MemoryType::Ram,
    ///     MemoryAttributes::default(),
    /// );
    /// 
    /// memory_map.add_region(region).unwrap();
    /// ```
    pub fn add_region(&mut self, region: MemoryRegion) -> Result<()> {
        if self.region_count >= 16 {
            return Err(Error::OutOfMemory);
        }
        
        // Check for overlaps
        // 检查重叠
        for i in 0..self.region_count {
            if let Some(existing) = &self.regions[i] {
                if region.overlaps(existing) {
                    return Err(Error::InvalidArgument);
                }
            }
        }

        self.regions[self.region_count] = Some(region);
        self.region_count += 1;
        Ok(())
    }

    /// Find a memory region containing the given address
    /// 找到包含给定地址的内存区域
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::{MemoryMap, MemoryRegion, MemoryType, MemoryAttributes};
    /// 
    /// let mut memory_map = MemoryMap::new();
    /// 
    /// let region = MemoryRegion::new(
    ///     0x1000,
    ///     0x1000,
    ///     MemoryType::Ram,
    ///     MemoryAttributes::default(),
    /// );
    /// 
    /// memory_map.add_region(region).unwrap();
    /// 
    /// let found_region = memory_map.find_region(0x1500);
    /// assert!(found_region.is_some());
    /// ```
    pub fn find_region(&self, addr: usize) -> Option<&MemoryRegion> {
        for i in 0..self.region_count {
            if let Some(region) = &self.regions[i] {
                if region.contains(addr) {
                    return Some(region);
                }
            }
        }
        None
    }

    /// Get all RAM regions
    /// 获取所有RAM区域
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::{MemoryMap, MemoryRegion, MemoryType, MemoryAttributes};
    /// 
    /// let mut memory_map = MemoryMap::new();
    /// 
    /// // Add RAM region
    /// let ram_region = MemoryRegion::new(
    ///     0x1000,
    ///     0x1000,
    ///     MemoryType::Ram,
    ///     MemoryAttributes::default(),
    /// );
    /// memory_map.add_region(ram_region).unwrap();
    /// 
    /// // Add ROM region
    /// let rom_region = MemoryRegion::new(
    ///     0x2000,
    ///     0x1000,
    ///     MemoryType::Rom,
    ///     MemoryAttributes::default(),
    /// );
    /// memory_map.add_region(rom_region).unwrap();
    /// 
    /// // Get only RAM regions
    /// let ram_regions: Vec<_> = memory_map.ram_regions().collect();
    /// assert_eq!(ram_regions.len(), 1);
    /// ```
    pub fn ram_regions(&self) -> impl Iterator<Item = &MemoryRegion> {
        self.regions[0..self.region_count]
            .iter()
            .filter_map(|r| r.as_ref())
            .filter(|r| r.memory_type == MemoryType::Ram)
    }

    /// Get total RAM size
    /// 获取总RAM大小
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::{MemoryMap, MemoryRegion, MemoryType, MemoryAttributes};
    /// 
    /// let mut memory_map = MemoryMap::new();
    /// 
    /// let ram_region = MemoryRegion::new(
    ///     0x1000,
    ///     0x1000,
    ///     MemoryType::Ram,
    ///     MemoryAttributes::default(),
    /// );
    /// memory_map.add_region(ram_region).unwrap();
    /// 
    /// assert_eq!(memory_map.total_ram(), 0x1000);
    /// ```
    pub fn total_ram(&self) -> usize {
        self.ram_regions().map(|r| r.size).sum()
    }

    /// Get all regions
    /// 获取所有区域
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::{MemoryMap, MemoryRegion, MemoryType, MemoryAttributes};
    /// 
    /// let mut memory_map = MemoryMap::new();
    /// 
    /// // Add regions
    /// // ...
    /// 
    /// // Get all regions
    /// let regions: Vec<_> = memory_map.regions().collect();
    /// println!("Found {} memory regions", regions.len());
    /// ```
    pub fn regions(&self) -> impl Iterator<Item = &MemoryRegion> {
        self.regions[0..self.region_count]
            .iter()
            .filter_map(|r| r.as_ref())
    }
}

/// Page allocator interface
/// 页面分配器接口
pub trait PageAllocator {
    /// Allocate a page of memory
    /// 分配一页内存
    fn allocate_page(&mut self) -> Result<usize>;
    
    /// Free a page of memory
    /// 释放一页内存
    fn free_page(&mut self, addr: usize) -> Result<()>;
    
    /// Get page size
    /// 获取页面大小
    fn page_size(&self) -> usize;
}

/// Simple bitmap page allocator
/// 基于位图的简单页面分配器
pub struct BitmapPageAllocator {
    bitmap: &'static mut [u8],
    page_size: usize,
    total_pages: usize,
    free_pages: usize,
}

impl BitmapPageAllocator {
    /// Create a new bitmap page allocator
    /// 创建一个新的位图页面分配器
    /// 
    /// # Example
    /// ```
    /// use feathercore_common::mm::BitmapPageAllocator;
    /// 
    /// let mut bitmap = [0u8; 16]; // 128 pages
    /// let page_size = 4096; // 4KB pages
    /// let total_pages = 128;
    /// 
    /// let mut allocator = BitmapPageAllocator::new(&mut bitmap, page_size, total_pages);
    /// ```
    pub fn new(bitmap: &'static mut [u8], page_size: usize, total_pages: usize) -> Self {
        // Initialize bitmap to all free (0)
        // 将位图初始化为全部空闲（0）
        for byte in bitmap.iter_mut() {
            *byte = 0;
        }
        
        Self {
            bitmap,
            page_size,
            total_pages,
            free_pages: total_pages,
        }
    }
    
    /// Find a free page
    /// 找到一个空闲页面
    fn find_free_page(&self) -> Option<usize> {
        for (byte_idx, byte) in self.bitmap.iter().enumerate() {
            if *byte != 0xFF {
                for bit_idx in 0..8 {
                    let page_idx = byte_idx * 8 + bit_idx;
                    if page_idx < self.total_pages && (byte & (1 << bit_idx)) == 0 {
                        return Some(page_idx);
                    }
                }
            }
        }
        None
    }
}

impl PageAllocator for BitmapPageAllocator {
    fn allocate_page(&mut self) -> Result<usize> {
        if let Some(page_idx) = self.find_free_page() {
            let byte_idx = page_idx / 8;
            let bit_idx = page_idx % 8;
            
            // Mark page as allocated
            // 标记页面为已分配
            self.bitmap[byte_idx] |= 1 << bit_idx;
            self.free_pages -= 1;
            
            Ok(page_idx * self.page_size)
        } else {
            Err(Error::OutOfMemory)
        }
    }
    
    fn free_page(&mut self, addr: usize) -> Result<()> {
        let page_idx = addr / self.page_size;
        if page_idx >= self.total_pages {
            return Err(Error::InvalidArgument);
        }
        
        let byte_idx = page_idx / 8;
        let bit_idx = page_idx % 8;
        
        // Check if page is actually allocated
        // 检查页面是否真的被分配
        if (self.bitmap[byte_idx] & (1 << bit_idx)) == 0 {
            return Err(Error::InvalidArgument);
        }
        
        // Mark page as free
        // 标记页面为空闲
        self.bitmap[byte_idx] &= !(1 << bit_idx);
        self.free_pages += 1;
        
        Ok(())
    }
    
    fn page_size(&self) -> usize {
        self.page_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bump_allocator() {
        let mut buffer = [0u8; 1024];
        let mut allocator = BumpAllocator::new(buffer.as_ptr() as usize, buffer.len());
        
        let layout = Layout::from_size_align(64, 8).unwrap();
        let ptr = allocator.allocate(layout).unwrap();
        
        assert_eq!(ptr.as_ptr() as usize % 8, 0);
        assert_eq!(allocator.used(), 64);
        assert_eq!(allocator.available(), 1024 - 64);
    }
    
    #[test]
    fn test_memory_region() {
        let region = MemoryRegion::new(
            0x1000,
            0x1000,
            MemoryType::Ram,
            MemoryAttributes {
                readable: true,
                writable: true,
                executable: true,
                ..Default::default()
            },
        );
        
        assert!(region.contains(0x1000));
        assert!(region.contains(0x1FFF));
        assert!(!region.contains(0x2000));
        assert_eq!(region.end(), 0x2000);
    }
}