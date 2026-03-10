//! No-std async runtime for FeatherCore
//!
//! This module provides a minimal async runtime that can be used in both
//! bootloader and kernel contexts. The runtime is designed to be:
//! 1. No-std compatible
//! 2. Minimal footprint
//! 3. Extensible for different execution contexts
//!
//! In bootloader context: Bare-metal async tasks
//! In kernel context: Thread-based async tasks

#![cfg(feature = "async")]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use core::sync::atomic::{AtomicBool, Ordering};

use crate::Error;
use crate::Result;

/// Task state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TaskState {
    /// Task is ready to run
    Ready,
    /// Task is waiting
    Waiting,
    /// Task has completed
    Completed,
    /// Task has failed
    Failed(Error),
}

/// Task identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(usize);

/// Async task trait
pub trait AsyncTask {
    /// Run the task to completion
    fn run(&mut self) -> impl Future<Output = Result<()>>;
}

/// Simple waker for no_std environment
pub struct SimpleWaker {
    woken: AtomicBool,
}

impl SimpleWaker {
    /// Create a new simple waker
    pub const fn new() -> Self {
        Self {
            woken: AtomicBool::new(false),
        }
    }

    /// Wake the waker
    pub fn wake(&self) {
        self.woken.store(true, Ordering::Release);
    }

    /// Check if woken
    pub fn is_woken(&self) -> bool {
        self.woken.load(Ordering::Acquire)
    }

    /// Reset woken state
    pub fn reset(&self) {
        self.woken.store(false, Ordering::Release);
    }
}

impl Waker for &SimpleWaker {
    fn wake(self: Pin<&mut Self>) {
        self.wake();
    }

    fn wake_by_ref(self: Pin<&Self>) {
        self.wake();
    }
}

/// Basic async executor for no_std environment
pub struct AsyncExecutor {
    tasks: [Option<TaskEntry>; 32],
    task_count: usize,
    waker: SimpleWaker,
}

struct TaskEntry {
    id: TaskId,
    state: TaskState,
    future: Pin<Box<dyn Future<Output = Result<()>>>>,
}

impl AsyncExecutor {
    /// Create a new async executor
    pub fn new() -> Self {
        Self {
            tasks: [const { None }; 32],
            task_count: 0,
            waker: SimpleWaker::new(),
        }
    }

    /// Spawn a new async task
    pub fn spawn<F>(&mut self, future: F) -> Result<TaskId>
    where
        F: Future<Output = Result<()>> + 'static,
    {
        if self.task_count >= 32 {
            return Err(Error::OutOfMemory);
        }
        
        let id = TaskId(self.task_count);
        self.tasks[self.task_count] = Some(TaskEntry {
            id,
            state: TaskState::Ready,
            future: Box::pin(future),
        });
        self.task_count += 1;
        Ok(id)
    }

    /// Run the executor until all tasks are complete
    pub fn run(&mut self) -> Result<()> {
        loop {
            let mut all_completed = true;
            let mut made_progress = false;

            for i in 0..self.task_count {
                if let Some(ref mut task) = &mut self.tasks[i] {
                    match task.state {
                    TaskState::Ready => {
                        all_completed = false;
                        
                        // Create a context with our simple waker
                        let waker = unsafe {
                            core::task::Waker::from_raw(
                                core::task::RawWaker::new(
                                    &self.waker as *const _ as *const (),
                                    &RAW_WAKER_VTABLE,
                                )
                            )
                        };
                        let mut cx = Context::from_waker(&waker);

                        // Poll the future
                        match task.future.as_mut().poll(&mut cx) {
                            Poll::Ready(Ok(())) => {
                                task.state = TaskState::Completed;
                                made_progress = true;
                            }
                            Poll::Ready(Err(e)) => {
                                task.state = TaskState::Failed(e);
                                made_progress = true;
                            }
                            Poll::Pending => {
                                task.state = TaskState::Waiting;
                            }
                        }
                    }
                    TaskState::Waiting => {
                        all_completed = false;
                        if self.waker.is_woken() {
                            task.state = TaskState::Ready;
                            self.waker.reset();
                            made_progress = true;
                        }
                    }
                    TaskState::Completed | TaskState::Failed(_) => {
                        // Task is already done
                    }
                }
            }
        }

            if all_completed {
                break;
            }

            if !made_progress {
                // No progress was made, we need to wait for an event
                // In a real implementation, this would put the CPU to sleep
                // or wait for interrupts
                core::hint::spin_loop();
            }
        }

        // Check for any failed tasks
        for i in 0..self.task_count {
            if let Some(ref task) = &self.tasks[i] {
                if let TaskState::Failed(e) = task.state {
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    /// Get the number of tasks
    pub fn task_count(&self) -> usize {
        self.task_count
    }
}

// Raw waker vtable for our simple waker
static RAW_WAKER_VTABLE: core::task::RawWakerVTable = {
    unsafe fn clone(data: *const ()) -> core::task::RawWaker {
        core::task::RawWaker::new(data, &RAW_WAKER_VTABLE)
    }
    
    unsafe fn wake(data: *const ()) {
        let waker = &*(data as *const SimpleWaker);
        waker.wake();
    }
    
    unsafe fn wake_by_ref(data: *const ()) {
        let waker = &*(data as *const SimpleWaker);
        waker.wake();
    }
    
    unsafe fn drop(_data: *const ()) {
        // Nothing to drop
    }
    
    core::task::RawWakerVTable::new(clone, wake, wake_by_ref, drop)
};

/// Async delay function
pub async fn delay(cycles: usize) {
    struct DelayFuture {
        cycles: usize,
        current: usize,
    }

    impl Future for DelayFuture {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.current >= self.cycles {
                Poll::Ready(())
            } else {
                self.current += 1;
                Poll::Pending
            }
        }
    }

    DelayFuture {
        cycles,
        current: 0,
    }.await
}

/// Async yield function
pub async fn yield_now() {
    struct YieldFuture {
        yielded: bool,
    }

    impl Future for YieldFuture {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.yielded {
                Poll::Ready(())
            } else {
                self.yielded = true;
                Poll::Pending
            }
        }
    }

    YieldFuture { yielded: false }.await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_executor() {
        let mut executor = AsyncExecutor::new();
        
        // Spawn a simple async task
        let task = async {
            delay(10).await;
            Ok(())
        };
        
        assert!(executor.spawn(task).is_ok());
        assert_eq!(executor.task_count(), 1);
        
        // Run the executor
        assert!(executor.run().is_ok());
    }
}