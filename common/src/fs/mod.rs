//! Filesystem framework for FeatherCore
//! FeatherCore 文件系统框架
//! 
//! This module provides a virtual filesystem (VFS) framework for FeatherCore.
//! 此模块为 FeatherCore 提供虚拟文件系统 (VFS) 框架。

/// File system trait
/// 文件系统 trait
pub trait FileSystem {
    /// Initialize the filesystem
    /// 初始化文件系统
    fn init(&self) -> crate::Result<()>;
    
    /// Get the filesystem name
    /// 获取文件系统名称
    fn name(&self) -> &str;
}

/// Virtual File System (VFS) manager
/// 虚拟文件系统 (VFS) 管理器
pub struct VfsManager {
    filesystems: [Option<&'static dyn FileSystem>; 8],
    fs_count: usize,
}

impl VfsManager {
    /// Create a new VFS manager
    /// 创建一个新的 VFS 管理器
    pub const fn new() -> Self {
        Self {
            filesystems: [None; 8],
            fs_count: 0,
        }
    }
    
    /// Add a filesystem to the manager
    /// 向管理器添加文件系统
    pub fn add_filesystem(&mut self, fs: &'static dyn FileSystem) -> crate::Result<()> {
        if self.fs_count >= 8 {
            return Err(crate::Error::OutOfMemory);
        }
        
        self.filesystems[self.fs_count] = Some(fs);
        self.fs_count += 1;
        Ok(())
    }
    
    /// Initialize all filesystems
    /// 初始化所有文件系统
    pub fn init_all(&self) -> crate::Result<()> {
        for i in 0..self.fs_count {
            if let Some(fs) = &self.filesystems[i] {
                fs.init()?;
            }
        }
        Ok(())
    }
    
    /// Get the number of filesystems
    /// 获取文件系统数量
    pub fn fs_count(&self) -> usize {
        self.fs_count
    }
}