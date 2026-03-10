//! Memory management utilities for no_std environment
//!
//! This module provides basic memory management functionality that can be used
//! in both bootloader and kernel contexts.

#![cfg(feature = "mm")]

use core::alloc::Layout;
use core::ptr::NonNull;

use crate::Error;
use crate::Result;

/// Simple bump allocator for bootloader and early kernel initialization
pub struct BumpAllocator {
    start: usize,
    end: usize,
    current: usize,
}

impl BumpAllocator {
    /// Create a new bump allocator with the given memory region
    pub const fn new(start: usize, size: usize) -> Self {
        Self {
            start,
            end: start + size,
            current: start,
        }
    }

    /// Allocate memory with the given layout
    pub fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>> {
        let aligned_addr = crate::util::align_up(self.current, layout.align());
        let new_current = aligned_addr + layout.size();

        if new_current > self.end {
            return Err(Error::OutOfMemory);
        }

        self.current = new_current;

        // Use safe NonNull::new instead of unsafe new_unchecked
        NonNull::new(aligned_addr as *mut u8)
            .ok_or(Error::OutOfMemory)
    }

    /// Reset the allocator to its initial state
    pub fn reset(&mut self) {
        self.current = self.start;
    }

    /// Get the amount of memory used
    pub fn used(&self) -> usize {
        self.current - self.start
    }

    /// Get the amount of memory available
    pub fn available(&self) -> usize {
        self.end - self.current
    }

    /// Get the total memory size
    pub fn total(&self) -> usize {
        self.end - self.start
    }
}



/// Memory region descriptor
#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    /// Start address of the region
    pub start: usize,
    /// Size of the region in bytes
    pub size: usize,
    /// Memory type
    pub memory_type: MemoryType,
    /// Memory attributes
    pub attributes: MemoryAttributes,
}

/// Memory type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    /// RAM (read-write executable)
    Ram,
    /// ROM (read-only executable)
    Rom,
    /// Device memory (read-write, non-executable)
    Device,
    /// Reserved memory (not usable)
    Reserved,
}

/// Memory attributes
#[derive(Debug, Clone, Copy, Default)]
pub struct MemoryAttributes {
    /// Readable
    pub readable: bool,
    /// Writable
    pub writable: bool,
    /// Executable
    pub executable: bool,
    /// Cacheable
    pub cacheable: bool,
    /// Shareable
    pub shareable: bool,
}

impl MemoryRegion {
    /// Create a new memory region
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
    pub fn contains(&self, addr: usize) -> bool {
        addr >= self.start && addr < self.start + self.size
    }

    /// Get the end address (exclusive)
    pub fn end(&self) -> usize {
        self.start + self.size
    }

    /// Check if this region overlaps with another region
    pub fn overlaps(&self, other: &MemoryRegion) -> bool {
        self.start < other.end() && other.start < self.end()
    }
}

/// Memory map for the system
pub struct MemoryMap {
    regions: [Option<MemoryRegion>; 16],
    region_count: usize,
}

impl MemoryMap {
    /// Create a new empty memory map
    pub fn new() -> Self {
        Self {
            regions: [None; 16],
            region_count: 0,
        }
    }

    /// Add a memory region to the map
    pub fn add_region(&mut self, region: MemoryRegion) -> Result<()> {
        if self.region_count >= 16 {
            return Err(Error::OutOfMemory);
        }
        
        // Check for overlaps
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
    pub fn ram_regions(&self) -> impl Iterator<Item = &MemoryRegion> {
        self.regions[0..self.region_count]
            .iter()
            .filter_map(|r| r.as_ref())
            .filter(|r| r.memory_type == MemoryType::Ram)
    }

    /// Get total RAM size
    pub fn total_ram(&self) -> usize {
        self.ram_regions().map(|r| r.size).sum()
    }

    /// Get all regions
    pub fn regions(&self) -> impl Iterator<Item = &MemoryRegion> {
        self.regions[0..self.region_count]
            .iter()
            .filter_map(|r| r.as_ref())
    }
}

/// Page allocator interface
pub trait PageAllocator {
    /// Allocate a page of memory
    fn allocate_page(&mut self) -> Result<usize>;
    
    /// Free a page of memory
    fn free_page(&mut self, addr: usize) -> Result<()>;
    
    /// Get page size
    fn page_size(&self) -> usize;
}

/// Simple bitmap page allocator
pub struct BitmapPageAllocator {
    bitmap: &'static mut [u8],
    page_size: usize,
    total_pages: usize,
    free_pages: usize,
}

impl BitmapPageAllocator {
    /// Create a new bitmap page allocator
    pub fn new(bitmap: &'static mut [u8], page_size: usize, total_pages: usize) -> Self {
        // Initialize bitmap to all free (0)
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
        if (self.bitmap[byte_idx] & (1 << bit_idx)) == 0 {
            return Err(Error::InvalidArgument);
        }
        
        // Mark page as free
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