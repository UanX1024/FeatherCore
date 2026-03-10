use core::ptr::NonNull;
use core::mem::MaybeUninit;
use core::future::Future;
use core::pin::Pin;
use alloc::boxed::Box;

/// Task state
#[derive(Debug, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Suspended,
    Terminated,
}

/// Task type
pub enum TaskType {
    /// Synchronous task with a function entry point
    Sync { entry_point: NonNull<()> },
    /// Asynchronous task with a future
    Async { future: Pin<Box<dyn Future<Output = ()> + 'static>> },
}

/// Task structure
pub struct Task {
    pub id: usize,
    pub name: &'static str,
    pub state: TaskState,
    pub priority: u8,
    pub stack_ptr: NonNull<u8>,
    pub thread_id: usize, // 线程ID，用于标识任务所属的线程
    task_type: TaskType,
}

// Safe to implement Send since we're handling task synchronization explicitly
unsafe impl Send for Task {}

impl Task {
    /// Create a new synchronous task
    pub fn new_sync(
        id: usize,
        name: &'static str,
        priority: u8,
        stack_ptr: NonNull<u8>,
        thread_id: usize,
        entry_point: NonNull<()>,
    ) -> Self {
        Task {
            id,
            name,
            state: TaskState::Ready,
            priority,
            stack_ptr,
            thread_id,
            task_type: TaskType::Sync { entry_point },
        }
    }
    
    /// Create a new asynchronous task
    pub fn new_async(
        id: usize,
        name: &'static str,
        priority: u8,
        stack_ptr: NonNull<u8>,
        thread_id: usize,
        future: impl Future<Output = ()> + 'static,
    ) -> Self {
        Task {
            id,
            name,
            state: TaskState::Ready,
            priority,
            stack_ptr,
            thread_id,
            task_type: TaskType::Async { future: Box::pin(future) },
        }
    }
    
    /// Set task state
    pub fn set_state(&mut self, state: TaskState) {
        self.state = state;
    }
    
    /// Check if the task is asynchronous
    pub fn is_async(&self) -> bool {
        matches!(self.task_type, TaskType::Async { .. })
    }
    
    /// Get the entry point of a synchronous task
    pub fn entry_point(&self) -> Option<NonNull<()>> {
        match &self.task_type {
            TaskType::Sync { entry_point } => Some(*entry_point),
            _ => None,
        }
    }
    
    /// Get a mutable reference to an async task's future
    pub fn async_future_mut(&mut self) -> Option<&mut core::pin::Pin<Box<dyn core::future::Future<Output = ()> + 'static>>> {
        match &mut self.task_type {
            TaskType::Async { future } => Some(future),
            _ => None,
        }
    }
}

/// Task stack
pub struct TaskStack {
    stack: MaybeUninit<[u8; 4096]>,
}

impl TaskStack {
    /// Create a new task stack
    pub const fn new() -> Self {
        TaskStack {
            stack: MaybeUninit::uninit(),
        }
    }
    
    /// Get the stack pointer
    pub fn stack_ptr(&self) -> NonNull<u8> {
        let ptr = self.stack.as_ptr() as *mut u8;
        unsafe {
            // Return pointer to the top of the stack
            NonNull::new_unchecked(ptr.add(4096))
        }
    }
}
