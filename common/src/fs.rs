//! Filesystem Framework
//! 文件系统框架
//!
//! This module provides the virtual filesystem (VFS) interface.
//! 该模块提供虚拟文件系统 (VFS) 接口。

use crate::Result;

/// VFS error types / VFS 错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VfsError {
    /// Not found / 未找到
    NotFound,
    /// Permission denied / 权限拒绝
    PermissionDenied,
    /// Already exists / 已存在
    AlreadyExists,
    /// Is a directory / 是目录
    IsDirectory,
    /// Not a directory / 不是目录
    NotDirectory,
    /// Invalid name / 无效名称
    InvalidName,
    /// Directory not empty / 目录非空
    DirectoryNotEmpty,
    /// Read-only filesystem / 只读文件系统
    ReadOnly,
    /// No space left / 空间不足
    NoSpace,
}

impl core::fmt::Display for VfsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            VfsError::NotFound => write!(f, "Not found"),
            VfsError::PermissionDenied => write!(f, "Permission denied"),
            VfsError::AlreadyExists => write!(f, "Already exists"),
            VfsError::IsDirectory => write!(f, "Is a directory"),
            VfsError::NotDirectory => write!(f, "Not a directory"),
            VfsError::InvalidName => write!(f, "Invalid name"),
            VfsError::DirectoryNotEmpty => write!(f, "Directory not empty"),
            VfsError::ReadOnly => write!(f, "Read-only filesystem"),
            VfsError::NoSpace => write!(f, "No space left"),
        }
    }
}

/// File type / 文件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    /// Regular file / 普通文件
    RegularFile,
    /// Directory / 目录
    Directory,
    /// Symbolic link / 符号链接
    SymLink,
    /// Block device / 块设备
    BlockDevice,
    /// Character device / 字符设备
    CharDevice,
    /// Named pipe / 命名管道
    Fifo,
    /// Socket / 套接字
    Socket,
}

/// File mode / 文件模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FileMode(pub u16);

impl FileMode {
    /// Read permission / 读权限
    pub const RUSR: Self = Self(0o400);
    /// Write permission / 写权限
    pub const WUSR: Self = Self(0o200);
    /// Execute permission / 执行权限
    pub const XUSR: Self = Self(0o100);
    /// Read for group / 组读权限
    pub const RGRP: Self = Self(0o040);
    /// Write for group / 组写权限
    pub const WGRP: Self = Self(0o020);
    /// Execute for group / 组执行权限
    pub const XGRP: Self = Self(0o010);
    /// Read for others / 其他读权限
    pub const ROTH: Self = Self(0o004);
    /// Write for others / 其他写权限
    pub const WOTH: Self = Self(0o002);
    /// Execute for others / 其他执行权限
    pub const XOTH: Self = Self(0o001);
}

/// File statistics / 文件统计信息
#[derive(Debug, Clone, Default)]
pub struct FileStat {
    /// Device ID / 设备 ID
    pub st_dev: u64,
    /// Inode number / Inode 编号
    pub st_ino: u64,
    /// File type / 文件类型
    pub st_mode: FileMode,
    /// Number of hard links / 硬链接数
    pub st_nlink: u32,
    /// User ID / 用户 ID
    pub st_uid: u32,
    /// Group ID / 组 ID
    pub st_gid: u32,
    /// File size / 文件大小
    pub st_size: u64,
    /// Last access time / 最后访问时间
    pub st_atime: u64,
    /// Last modification time / 最后修改时间
    pub st_mtime: u64,
    /// Last change time / 最后改变时间
    pub st_ctime: u64,
}

/// File trait for filesystem operations
/// 文件特征用于文件系统操作
pub trait File {
    /// Read data from file / 从文件读取数据
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    /// Write data to file / 向文件写入数据
    fn write(&mut self, buf: &[u8]) -> Result<usize>;

    /// Seek to position / 查找位置
    fn seek(&mut self, pos: u64) -> Result<u64>;

    /// Get file statistics / 获取文件统计信息
    fn stat(&mut self) -> Result<FileStat>;
}

/// Directory entry / 目录项
#[derive(Debug, Clone)]
pub struct DirEntry {
    /// Entry name / 条目名称
    pub name: String,
    /// File type / 文件类型
    pub file_type: FileType,
    /// Inode number / Inode 编号
    pub ino: u64,
}

/// Dir trait for directory operations
/// 目录特征用于目录操作
pub trait Dir {
    /// Read directory entry / 读取目录项
    fn read_entry(&mut self) -> Result<Option<DirEntry>>;

    /// Create directory / 创建目录
    fn create_dir(&mut self, name: &str, mode: FileMode) -> Result<()>;

    /// Remove directory / 删除目录
    fn remove_dir(&mut self, name: &str) -> Result<()>;

    /// Create file / 创建文件
    fn create_file(&mut self, name: &str, mode: FileMode) -> Result<()>;

    /// Remove file / 删除文件
    fn remove_file(&mut self, name: &str) -> Result<()>;

    /// Rename file or directory / 重命名文件或目录
    fn rename(&mut self, old_name: &str, new_name: &str) -> Result<()>;
}
