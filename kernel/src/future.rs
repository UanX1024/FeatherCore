use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use core::marker::Unpin;
use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::sync::Arc;
use crate::sync::Mutex;
use crate::sched::wake_task;
use crate::time;

/// Async task context
pub struct AsyncTaskContext {
    future: Pin<Box<dyn Future<Output = ()> + 'static>>,
    waker: Option<Waker>,
    priority: u8, // 任务优先级，0-255，值越小优先级越高
}

// Safe to implement Send and Sync since futures are Send and Waker is Send + Sync
unsafe impl Send for AsyncTaskContext {}
unsafe impl Sync for AsyncTaskContext {}

/// Async executor with priority queue
pub struct AsyncExecutor {
    tasks: Vec<AsyncTaskContext>,
    // 使用Vec作为简单的优先级队列，实际应用中可以使用更高效的数据结构
}

// Safe to implement Send and Sync since it only contains Send + Sync types
unsafe impl Send for AsyncExecutor {}
unsafe impl Sync for AsyncExecutor {}

impl AsyncExecutor {
    /// Create a new async executor
    pub const fn new() -> Self {
        AsyncExecutor {
            tasks: Vec::new(),
        }
    }
    
    /// Spawn a new async task with priority
    pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static, priority: u8) -> usize {
        let task_id = self.tasks.len();
        self.tasks.push(AsyncTaskContext {
            future: Box::pin(future),
            waker: None,
            priority,
        });
        task_id
    }
    
    /// Poll all ready tasks in priority order
    pub fn poll(&mut self) {
        // 按照优先级排序任务（值越小优先级越高）
        let mut sorted_tasks: Vec<_> = self.tasks.iter_mut().enumerate().collect();
        sorted_tasks.sort_by(|a, b| a.1.priority.cmp(&b.1.priority));
        
        // 轮询所有任务
        for (i, task) in sorted_tasks {
            if task.waker.is_none() {
                task.waker = Some(create_waker(i));
            }
            
            let waker = task.waker.as_ref().unwrap();
            let mut cx = Context::from_waker(waker);
            
            match task.future.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    // Task completed, mark for removal
                    // TODO: Implement task cleanup
                },
                Poll::Pending => {
                    // Task still pending, will be woken later
                },
            }
        }
    }
}

/// Waker data structure
struct WakerData {
    task_id: usize,
}

/// Waker virtual table
static WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    |data| {
        // Clone the waker
        let waker_data = unsafe { &*(data as *const WakerData) };
        let cloned_data = Box::new(WakerData { task_id: waker_data.task_id });
        RawWaker::new(Box::into_raw(cloned_data) as *const (), &WAKER_VTABLE)
    },
    |data| {
        // Wake the task
        let waker_data = unsafe { &*(data as *const WakerData) };
        // Call the scheduler to wake the task
        wake_task(waker_data.task_id);
    },
    |data| {
        // Wake the task (for waker reference)
        let waker_data = unsafe { &*(data as *const WakerData) };
        // Call the scheduler to wake the task
        wake_task(waker_data.task_id);
    },
    |data| {
        // Drop the waker
        unsafe {
            let _ = Box::from_raw(data as *mut WakerData);
        }
    },
);

/// Create a waker for a task
pub fn create_waker(task_id: usize) -> Waker {
    let waker_data = Box::new(WakerData { task_id });
    let raw_waker = RawWaker::new(Box::into_raw(waker_data) as *const (), &WAKER_VTABLE);
    unsafe {
        Waker::from_raw(raw_waker)
    }
}

/// Global async executor
static ASYNC_EXECUTOR: Mutex<AsyncExecutor> = Mutex::new(AsyncExecutor::new());

/// Spawn an async task with default priority (128)
pub fn spawn(future: impl Future<Output = ()> + 'static) -> usize {
    spawn_with_priority(future, 128)
}

/// Spawn an async task with specific priority
pub fn spawn_with_priority(future: impl Future<Output = ()> + 'static, priority: u8) -> usize {
    ASYNC_EXECUTOR.lock().spawn(future, priority)
}

/// Poll all async tasks
pub fn poll_async_tasks() {
    ASYNC_EXECUTOR.lock().poll()
}

/// Async sleep future
// SleepFuture 结构体
pub struct SleepFuture {
    duration: u64,
    timer_id: Option<usize>,
    completed: bool,
}

// SleepFuture 实现
impl SleepFuture {
    /// 创建一个新的 SleepFuture
    pub fn new(duration: u64) -> Self {
        SleepFuture {
            duration,
            timer_id: None,
            completed: false,
        }
    }
}

// SleepFuture 的 Drop 实现
impl Drop for SleepFuture {
    fn drop(&mut self) {
        if let Some(timer_id) = self.timer_id {
            time::disable_timer(timer_id);
        }
    }
}

// 全局变量用于存储当前需要唤醒的 waker
// 使用静态变量替代 thread_local，因为在裸机环境中 thread_local 可能不可用
static CURRENT_WAKER: Mutex<Option<Waker>> = Mutex::new(None);

// 定时器回调函数，用于唤醒当前任务
fn wake_current_task() {
    if let Some(waker) = CURRENT_WAKER.lock().take() {
        waker.wake();
    }
}

// SleepFuture 的 Future 实现
impl Future for SleepFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 获取可变引用
        let this = self.get_mut();

        // 如果已经完成，直接返回 Ready
        if this.completed {
            return Poll::Ready(());
        }

        // 如果还没有设置定时器，设置它
        if this.timer_id.is_none() {
            // 分配一个定时器
            if let Some(timer_id) = time::allocate_timer() {
                // 保存 waker
                let waker = cx.waker().clone();
                
                // 保存当前任务ID到全局变量，以便回调函数可以访问
                *CURRENT_WAKER.lock() = Some(waker);
                
                // 配置并启用定时器
                time::set_timer_period(timer_id, this.duration as usize);
                time::set_timer_callback(timer_id, wake_current_task);
                time::enable_timer(timer_id);
                
                this.timer_id = Some(timer_id);
            } else {
                // 没有可用的定时器，返回 Pending
                return Poll::Pending;
            }
        }

        // 如果定时器已经触发，返回 Ready
        if this.completed {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

// 实现 poll_fn 函数
pub fn poll_fn<T, F>(f: F) -> impl Future<Output = T> + Unpin
where
    F: FnMut(&mut Context<'_>) -> Poll<T> + Unpin,
{
    struct PollFn<F>(F);

    impl<F: FnMut(&mut Context<'_>) -> Poll<T>, T> Future for PollFn<F> {
        type Output = T;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
            // 使用 Pin::get_unchecked_mut 绕过 Unpin 检查
            unsafe {
                Pin::get_unchecked_mut(self).0(cx)
            }
        }
    }

    PollFn(f)
}

/// Async yield future - 用于异步任务主动让出执行权
pub struct YieldFuture {} 

impl YieldFuture {
    /// Create a new yield future
    pub fn new() -> Self {
        YieldFuture {}
    }
}

impl Future for YieldFuture {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 立即唤醒当前任务，使其重新加入轮询队列的末尾
        cx.waker().wake_by_ref();
        
        // 返回Pending，让出执行权给其他任务
        Poll::Pending
    }
}

/// Async yield utility function - 方便使用的异步yield函数
pub async fn async_yield() {
    YieldFuture::new().await;
}

/// Async mutex guard
pub struct AsyncMutexGuard<'a, T> {
    mutex: &'a AsyncMutex<T>,
}

impl<'a, T> Drop for AsyncMutexGuard<'a, T> {
    fn drop(&mut self) {
        // Release the mutex
        self.mutex.unlock();
    }
}

impl<'a, T> core::ops::Deref for AsyncMutexGuard<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for AsyncMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

/// Async mutex implementation
pub struct AsyncMutex<T> {
    locked: core::sync::atomic::AtomicBool,
    data: core::cell::UnsafeCell<T>,
    waiters: Arc<Mutex<Vec<Waker>>>,
}

unsafe impl<T: Send> Send for AsyncMutex<T> {}
unsafe impl<T: Send> Sync for AsyncMutex<T> {}

impl<T> AsyncMutex<T> {
    /// Create a new async mutex
    pub fn new(data: T) -> Self {
        AsyncMutex {
            locked: core::sync::atomic::AtomicBool::new(false),
            data: core::cell::UnsafeCell::new(data),
            waiters: alloc::sync::Arc::new(crate::sync::Mutex::new(Vec::new())),
        }
    }
    
    /// Lock the mutex asynchronously
    pub async fn lock(&self) -> AsyncMutexGuard<'_, T> {
        // Try to lock the mutex immediately
        if !self.locked.swap(true, core::sync::atomic::Ordering::Acquire) {
            return AsyncMutexGuard { mutex: self };
        }
        
        // Wait until the mutex is available
        let _guard = crate::future::poll_fn(|cx| {
            // Check if the mutex is available again
            if !self.locked.swap(true, core::sync::atomic::Ordering::Acquire) {
                return Poll::Ready(());
            }
            
            // Add waker to wait list
            self.waiters.lock().push(cx.waker().clone());
            Poll::Pending
        }).await;
        
        AsyncMutexGuard { mutex: self }
    }
    
    /// Unlock the mutex
    fn unlock(&self) {
        // Unlock the mutex
        self.locked.store(false, core::sync::atomic::Ordering::Release);
        
        // Wake up one waiter if any
        if let Some(waker) = self.waiters.lock().pop() {
            waker.wake();
        }
    }
}