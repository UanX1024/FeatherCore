//! Filesystem framework for FeatherCore
//! 
//! This module provides a virtual filesystem (VFS) framework for FeatherCore.

#![no_std]

/// File system trait
pub trait FileSystem {
    /// Initialize the filesystem
    fn init(&self) -> crate::Result<()>;
    
    /// Get the filesystem name
    fn name(&self) -> &str;
}

/// Virtual File System (VFS) manager
pub struct VfsManager {
    filesystems: [Option<&'static dyn FileSystem>; 8],
    fs_count: usize,
}

impl VfsManager {
    /// Create a new VFS manager
    pub const fn new() -> Self {
        Self {
            filesystems: [None; 8],
            fs_count: 0,
        }
    }
    
    /// Add a filesystem to the manager
    pub fn add_filesystem(&mut self, fs: &'static dyn FileSystem) -> crate::Result<()> {
        if self.fs_count >= 8 {
            return Err(crate::Error::OutOfMemory);
        }
        
        self.filesystems[self.fs_count] = Some(fs);
        self.fs_count += 1;
        Ok(())
    }
    
    /// Initialize all filesystems
    pub fn init_all(&self) -> crate::Result<()> {
        for i in 0..self.fs_count {
            if let Some(fs) = &self.filesystems[i] {
                fs.init()?;
            }
        }
        Ok(())
    }
    
    /// Get the number of filesystems
    pub fn fs_count(&self) -> usize {
        self.fs_count
    }
}