use crate::task::{Task, TaskState};
use alloc::vec::Vec;
use crate::sync::Mutex;
use crate::info;
use core::pin::Pin;
use core::future::Future;
use core::task::{Context, Poll};
use crate::future;
use super::future::create_waker;

static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

/// 线程结构，用于管理同一线程内的任务
pub struct Thread {
    _id: usize,
    _name: &'static str,
    current_task: Option<usize>, // 当前线程正在执行的任务
    ready_queue: Vec<usize>,     // 线程内的就绪任务队列
}

/// Scheduler structure
pub struct Scheduler {
    tasks: Vec<Task>,
    threads: Vec<Thread>,
    current_thread: Option<usize>,
    _thread_count: usize,
}

impl Scheduler {
    /// Create a new thread
    pub fn create_thread(&mut self, name: &'static str) -> usize {
        let thread_id = self.threads.len();
        self.threads.push(Thread {
            _id: thread_id,
            _name: name,
            current_task: None,
            ready_queue: Vec::new(),
        });
        thread_id
    }
    
    /// Create a new scheduler
    pub const fn new() -> Self {
        Scheduler {
            tasks: Vec::new(),
            threads: Vec::new(),
            current_thread: None,
            _thread_count: 0,
        }
    }
    
    /// Add a new task to the scheduler
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    
    /// Find the next task to run in the current thread
    pub fn find_next_task_in_thread(&mut self, thread_id: usize) -> Option<usize> {
        let thread = &mut self.threads[thread_id];
        
        // 首先检查线程的就绪队列
        if let Some(task_id) = thread.ready_queue.pop() {
            if self.tasks[task_id].state == TaskState::Ready {
                return Some(task_id);
            }
        }
        
        // 如果就绪队列为空，使用优先级调度查找当前线程的就绪任务
        let mut highest_priority = u8::MAX;
        let mut selected_task = None;
        
        for (i, task) in self.tasks.iter().enumerate() {
            if task.state == TaskState::Ready && task.thread_id == thread_id {
                if task.priority < highest_priority {
                    highest_priority = task.priority;
                    selected_task = Some(i);
                }
            }
        }
        
        selected_task
    }
    
    /// Find the next thread to run (for preemption between threads)
    pub fn find_next_thread(&self) -> Option<usize> {
        // 简单的轮询调度
        let current = self.current_thread.unwrap_or(0);
        
        for i in 0..self.threads.len() {
            let idx = (current + i + 1) % self.threads.len();
            // 检查线程是否有就绪任务
            for task in &self.tasks {
                if task.state == TaskState::Ready && task.thread_id == idx {
                    return Some(idx);
                }
            }
        }
        
        None
    }
    
    /// Schedule the next task
    pub fn schedule(&mut self) {
        if let Some(current_thread_id) = self.current_thread {
            // 1. 首先轮询当前线程中所有就绪的异步任务
            self.poll_async_tasks_in_thread(current_thread_id);
            
            // 2. 在当前线程内查找下一个就绪任务
            if let Some(next_task) = self.find_next_task_in_thread(current_thread_id) {
                // 同一线程内切换任务，不需要线程上下文切换
                self.switch_task_in_thread(current_thread_id, next_task);
                return;
            }
        }
        
        // 3. 如果当前线程没有就绪任务，查找其他线程
        if let Some(next_thread_id) = self.find_next_thread() {
            // 线程间切换，需要保存当前线程状态并恢复目标线程状态
            self.switch_to_thread(next_thread_id);
            
            // 在新线程中查找第一个任务
            if let Some(next_task) = self.find_next_task_in_thread(next_thread_id) {
                self.switch_task_in_thread(next_thread_id, next_task);
            }
        }
    }
    
    /// Switch task within the same thread
    pub fn switch_task_in_thread(&mut self, thread_id: usize, task_id: usize) {
        let thread = &mut self.threads[thread_id];
        
        // Save current task state if there is one
        if let Some(current_task_id) = thread.current_task {
            let current_task = &mut self.tasks[current_task_id];
            if current_task.is_async() && current_task.state == TaskState::Running {
                // For async tasks, we just need to save their state in the future
                current_task.set_state(TaskState::Ready);
            } else if current_task.state == TaskState::Running {
                current_task.set_state(TaskState::Ready);
            }
        }
        
        // Set new task as current
        thread.current_task = Some(task_id);
        let new_task = &mut self.tasks[task_id];
        new_task.set_state(TaskState::Running);
        
        // For sync tasks, we need to switch the execution context
        if !new_task.is_async() {
            // TODO: Implement context switching for sync tasks
        }
    }
    
    /// Switch to a different thread
    pub fn switch_to_thread(&mut self, thread_id: usize) {
        // Save current thread state if there is one
        if let Some(current_thread_id) = self.current_thread {
            if current_thread_id != thread_id {
                // TODO: Implement thread context switching
            }
        }
        
        // Set new thread as current
        self.current_thread = Some(thread_id);
    }
    
    /// Poll all asynchronous tasks in the given thread
    pub fn poll_async_tasks_in_thread(&mut self, thread_id: usize) {
        // 收集当前线程中所有就绪的异步任务
        let mut ready_async_tasks: Vec<usize> = Vec::new();
        
        for (i, task) in self.tasks.iter().enumerate() {
            if task.state == TaskState::Ready && task.thread_id == thread_id && task.is_async() {
                ready_async_tasks.push(i);
            }
        }
        
        // 轮询每个异步任务
        for task_id in ready_async_tasks {
            let task = &mut self.tasks[task_id];
            
            if let Some(future) = task.async_future_mut() {
                // Create a waker for this task
                let waker = create_waker(task_id);
                let mut cx = Context::from_waker(&waker);
                
                // Poll the future
                match Pin::as_mut(future).poll(&mut cx) {
                    Poll::Ready(()) => {
                        // Task completed, set to terminated
                        task.set_state(TaskState::Terminated);
                    },
                    Poll::Pending => {
                        // Task still pending, block it until woken
                        task.set_state(TaskState::Blocked);
                    },
                    // 当任务返回Pending时，当前线程可以继续执行其他任务
                    // 这实现了await不切换线程的特性
                }
            }
        }
    }
    
    /// Add a task to the ready queue of its thread
    pub fn add_to_ready_queue(&mut self, task_id: usize) {
        let thread_id = self.tasks[task_id].thread_id;
        self.threads[thread_id].ready_queue.push(task_id);
    }
    
    /// Switch to a specific task (deprecated, use switch_task_in_thread instead)
    pub fn switch_to(&mut self, task_id: usize) {
        // TODO: Implement task context switching
        if let Some(thread_id) = self.current_thread {
            self.switch_task_in_thread(thread_id, task_id);
        }
    }
    
    /// Wake up an asynchronous task
    pub fn wake_task(&mut self, task_id: usize) {
        if let Some(task) = self.tasks.get_mut(task_id) {
            // Only wake up blocked async tasks
            if task.is_async() && task.state == TaskState::Blocked {
                task.set_state(TaskState::Ready);
                // 将任务添加到所属线程的就绪队列
                self.add_to_ready_queue(task_id);
            }
        }
    }
}

/// Initialize the scheduler
pub fn init() {
    // 创建主线程
    SCHEDULER.lock().create_thread("main");
    // 设置当前线程为主线程
    SCHEDULER.lock().current_thread = Some(0);
    info!("Scheduler initialized with main thread");
}

/// Schedule the next task
pub fn schedule() {
    SCHEDULER.lock().schedule();
}

/// Create a new thread
pub fn create_thread(name: &'static str) -> usize {
    SCHEDULER.lock().create_thread(name)
}

/// Add a task to the scheduler
pub fn add_task(task: Task) {
    let mut scheduler = SCHEDULER.lock();
    scheduler.add_task(task);
    // 将新任务添加到所属线程的就绪队列
    let task_id = scheduler.tasks.len() - 1;
    scheduler.add_to_ready_queue(task_id);
}

/// Wake up an asynchronous task
pub fn wake_task(task_id: usize) {
    SCHEDULER.lock().wake_task(task_id);
}

/// Spawn an async task with default priority (128)
pub fn spawn_async(future: impl Future<Output = ()> + 'static) -> Result<usize, &'static str> {
    spawn_async_with_priority(future, 128)
}

/// Spawn an async task with specific priority
pub fn spawn_async_with_priority(future: impl Future<Output = ()> + 'static, priority: u8) -> Result<usize, &'static str> {
    // Add async task to the global executor with the given priority
    let task_id = future::spawn_with_priority(future, priority);
    
    // The executor will handle the task automatically, no need to wake a thread
    Ok(task_id)
}