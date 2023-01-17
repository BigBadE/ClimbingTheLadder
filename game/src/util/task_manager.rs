use std::any::Any;
use std::future;
use std::future::Future;
use std::pin::Pin;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use crate::error;

pub struct Task<T> {
    handle: JoinHandle<T>,
    after: fn(T)
}

pub struct TaskManager {
    cpu_runtime: Runtime,
    io_runtime: Runtime,
    tasks: Vec<Task<Box<dyn Any + Send>>>
}

impl TaskManager {
    pub fn new(cpu_runtime: Runtime, io_runtime: Runtime) -> Self {
        return Self {
            cpu_runtime,
            io_runtime,
            tasks: Vec::new()
        }
    }

    pub fn get_runtime(&self, io: bool) -> &Runtime {
        return if io {
            &self.io_runtime
        } else {
            &self.cpu_runtime
        }
    }

    pub fn queue<F>(&mut self, io_heavy: bool, task: F)
        where F: Future<Output=Box<(dyn Any + Send + 'static)>> + Send + 'static {
        if io_heavy {
            self.tasks.push(Task {
                handle: self.io_runtime.spawn(task),
                after: Self::empty,
            });
        } else {
            self.tasks.push(Task {
                handle: self.cpu_runtime.spawn(task),
                after: Self::empty,
            });
        }
    }

    pub fn queue_after<F>(&mut self, io_heavy: bool, task: F, after: fn(Box<(dyn Any + Send + 'static)>))
        where F: Future<Output=Box<(dyn Any + Send + 'static)>> + Send + 'static {
        if io_heavy {
            self.tasks.push(Task {
                handle: self.io_runtime.spawn(task),
                after, 
            });
        } else {
            self.tasks.push(Task {
                handle: self.cpu_runtime.spawn(task),
                after,
            });
        }
    }

    fn empty<T>(_: T) {

    }

    pub async fn poll(&mut self) -> Option<bool> {
        if !self.running() {
            return Some(false);
        }
        let task = self.tasks.pop().unwrap();
        if task.handle.is_finished() {
            match task.handle.await {
                Ok(result) => (task.after)(result),
                Err(error) => {
                    error!("Error with long task!:\n{}", error)
                }
            }
            return None;
        }
        return Some(true);
    }

    pub fn running(&self) -> bool {
        return !self.tasks.is_empty();
    }
}