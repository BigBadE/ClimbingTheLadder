use std::future::Future;
use tokio::runtime::{Handle, Runtime};
use tokio::task::JoinHandle;

use crate::{error, Game};
use crate::util::alloc_handle::AllocHandle;

pub struct Task {
    handle: JoinHandle<AllocHandle>,
    after: fn(&mut Game, &AllocHandle)
}

pub struct TaskManager {
    cpu_runtime: Handle,
    io_runtime: Handle,
    tasks: Vec<Task>
}

impl TaskManager {
    pub fn new(cpu_runtime: Handle, io_runtime: Handle) -> Self {
        return Self {
            cpu_runtime,
            io_runtime,
            tasks: Vec::new(),
        };
    }

    pub fn get_runtime(&self, io: bool) -> &Handle {
        return if io {
            &self.io_runtime
        } else {
            &self.cpu_runtime
        };
    }

    pub fn queue<F>(&mut self, io_heavy: bool, task: F)
        where F: Future<Output=AllocHandle> + Send + 'static {
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

    pub fn queue_after<F>(&mut self, io_heavy: bool, task: F, after: fn(&mut Game, &AllocHandle))
        where F: Future<Output=AllocHandle> + Send + 'static {
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

    fn empty(_: &mut Game, _: &AllocHandle) {}

    pub fn poll(&mut self) -> (bool, Option<FinishedTask>) {
        if !self.running() {
            return (true, None);
        }

        let task = self.tasks.get(0).unwrap();

        if task.handle.is_finished() {
            let task = self.tasks.pop().unwrap();
            return match self.cpu_runtime.block_on(task.handle) {
                Ok(result) => (true, Some(FinishedTask::new(result, task.after))),
                Err(error) => {
                    error!("Error running long task:\n{}", error);
                    self.poll()
                }
            }
        }

        return (false, None);
    }

    pub fn running(&self) -> bool {
        return !self.tasks.is_empty();
    }
}

pub struct FinishedTask {
    handle: AllocHandle,
    function: fn(&mut Game, &AllocHandle)
}

impl FinishedTask {
    pub fn new(handle: AllocHandle, function: fn(&mut Game, &AllocHandle)) -> Self {
        return FinishedTask {
            handle,
            function
        }
    }

    pub fn call(&self, game: &mut Game) {
        (self.function)(game, &self.handle);
    }
}