use core::ptr::NonNull;
use crate::sync::Mutex;
use crate::info;

/// Memory block header
#[repr(C)]
struct BlockHeader {
    size: usize,
    next: Option<NonNull<BlockHeader>>,
}

/// Memory allocator
struct Allocator {
    free_list: Option<NonNull<BlockHeader>>,
}

// Safe to implement Send and Sync since we're using spin::Mutex for thread safety
unsafe impl Send for Allocator {}
unsafe impl Sync for Allocator {}

impl Allocator {
    /// Create a new allocator
    pub const fn new() -> Self {
        Allocator {
            free_list: None,
        }
    }
    
    /// Initialize the allocator with a memory region
    pub unsafe fn init(&mut self, start: *mut u8, size: usize) {
        let block = start as *mut BlockHeader;
        (*block).size = size - core::mem::size_of::<BlockHeader>();
        (*block).next = None;
        self.free_list = NonNull::new(block);
    }
    
    /// Allocate memory
    pub unsafe fn allocate(&mut self, size: usize) -> Option<NonNull<u8>> {
        // Simple first-fit allocator
        let mut current = self.free_list;
        let mut prev: Option<NonNull<BlockHeader>> = None;
        
        while let Some(block) = current {
            if (*block.as_ptr()).size >= size {
                // Split the block if it's larger than needed
                let block_size = (*block.as_ptr()).size;
                if block_size > size + core::mem::size_of::<BlockHeader>() {
                    let new_block = (block.as_ptr() as *mut u8).add(core::mem::size_of::<BlockHeader>() + size) as *mut BlockHeader;
                    (*new_block).size = block_size - size - core::mem::size_of::<BlockHeader>();
                    (*new_block).next = (*block.as_ptr()).next;
                    (*block.as_ptr()).size = size;
                    (*block.as_ptr()).next = NonNull::new(new_block);
                }
                
                // Remove the block from the free list
                if let Some(prev_block) = prev {
                    (*prev_block.as_ptr()).next = (*block.as_ptr()).next;
                } else {
                    self.free_list = (*block.as_ptr()).next;
                }
                
                // Return pointer to the data
                let data_ptr = (block.as_ptr() as *mut u8).add(core::mem::size_of::<BlockHeader>());
                return NonNull::new(data_ptr);
            }
            
            prev = current;
            current = (*block.as_ptr()).next;
        }
        
        None // Out of memory
    }
    
    /// Free memory
    pub unsafe fn free(&mut self, ptr: NonNull<u8>) {
        // Get the block header
        let header_ptr = (ptr.as_ptr() as *mut u8).sub(core::mem::size_of::<BlockHeader>()) as *mut BlockHeader;
        let mut block = NonNull::new_unchecked(header_ptr);
        
        // Insert into free list in order
        let mut current = self.free_list;
        let mut prev = None;
        
        while let Some(current_block) = current {
            if current_block.as_ptr() > block.as_ptr() {
                break;
            }
            prev = current;
            current = (*current_block.as_ptr()).next;
        }
        
        // Check if we can coalesce with previous block
        if let Some(prev_block) = prev {
            let prev_end = (prev_block.as_ptr() as *mut u8).add(core::mem::size_of::<BlockHeader>() + (*prev_block.as_ptr()).size);
            if prev_end == block.as_ptr() as *mut u8 {
                // Coalesce with previous block
                (*prev_block.as_ptr()).size += core::mem::size_of::<BlockHeader>() + (*block.as_ptr()).size;
                (*prev_block.as_ptr()).next = (*block.as_ptr()).next;
                block = prev_block;
            } else {
                // Insert after previous block
                (*prev_block.as_ptr()).next = Some(block);
            }
        } else {
            // Insert at the beginning
            self.free_list = Some(block);
        }
        
        // Check if we can coalesce with next block
        if let Some(next_block) = (*block.as_ptr()).next {
            let block_end = (block.as_ptr() as *mut u8).add(core::mem::size_of::<BlockHeader>() + (*block.as_ptr()).size);
            if block_end == next_block.as_ptr() as *mut u8 {
                // Coalesce with next block
                (*block.as_ptr()).size += core::mem::size_of::<BlockHeader>() + (*next_block.as_ptr()).size;
                (*block.as_ptr()).next = (*next_block.as_ptr()).next;
            }
        }
    }
}

static ALLOCATOR: Mutex<Allocator> = Mutex::new(Allocator::new());

/// Initialize the memory manager
pub fn init() {
    // Initialize the allocator with a dummy memory region for now
    // In a real implementation, this would use actual RAM regions from memory.x
    unsafe {
        // Use a small static buffer as initial heap
        static mut HEAP_MEMORY: [u8; 4096] = [0; 4096];
        
        // Get mutable pointers for the static buffer
        let heap_ptr = HEAP_MEMORY.as_mut_ptr();
        let heap_size = HEAP_MEMORY.len();
        
        // Initialize the allocator
        ALLOCATOR.lock().init(heap_ptr, heap_size);
    }
    info!("Memory manager initialized");
}

/// Allocate memory
pub fn allocate(size: usize) -> Option<NonNull<u8>> {
    unsafe {
        ALLOCATOR.lock().allocate(size)
    }
}

/// Free memory
pub fn free(ptr: NonNull<u8>) {
    unsafe {
        ALLOCATOR.lock().free(ptr)
    }
}

/// Global allocator implementation
struct GlobalAllocator;

unsafe impl core::alloc::GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        match allocate(layout.size()) {
            Some(ptr) => ptr.as_ptr(),
            None => core::ptr::null_mut(),
        }
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        free(NonNull::new_unchecked(ptr));
    }
}

/// Register the global allocator
#[global_allocator]
static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator;

/// Allocation error handler
#[alloc_error_handler]
pub fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation failed: {:?}", layout);
}
