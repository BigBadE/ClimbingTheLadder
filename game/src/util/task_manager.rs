use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use tokio::runtime::Handle;
use tokio::task::JoinHandle;

use crate::{error, Game};
use crate::util::alloc_handle::AllocHandle;

pub struct Task {
    handle: JoinHandle<AllocHandle>,
    after: fn(&mut Game, AllocHandle),
}

pub struct TaskManager {
    cpu_runtime: Handle,
    io_runtime: Handle,
    tasks: Vec<Task>,
    empty_waker: Waker,
}

impl TaskManager {
    pub fn new(cpu_runtime: Handle, io_runtime: Handle) -> Self {
        return Self {
            cpu_runtime,
            io_runtime,
            tasks: Vec::new(),
            empty_waker: unsafe {
                Waker::from_raw(Self::make_empty(&() as *const ()))
            },
        };
    }

    fn make_empty(_: *const ()) -> RawWaker {
        return RawWaker::new(&() as *const (), &RawWakerVTable::new(Self::make_empty,
                                                                    Self::empty_fn, Self::empty_fn, Self::empty_fn_drop));
    }

    fn empty_fn(_: *const ()) {
        error!("Something tried to use empty waker!");
    }

    fn empty_fn_drop(_: *const ()) {}

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

    pub fn queue_after<F>(&mut self, io_heavy: bool, task: F, after: fn(&mut Game, AllocHandle))
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

    fn empty(_: &mut Game, _: AllocHandle) {}

    pub async fn poll(&mut self) -> (bool, Option<FinishedTask>) {
        if !self.running() {
            return (true, None);
        }

        let task: &mut Task = self.tasks.get_mut(0).unwrap();

        if task.handle.is_finished() {
            let task = self.tasks.pop().unwrap();
            //Mocks a poll to the finished task because it can't block_on.
            return match task.handle.await {
                Ok(result) => (true, Some(FinishedTask::new(result, task.after))),
                Err(error) => {
                    error!("Error running long task:\n{}", error);
                    self.tasks.pop();
                    (true, Some(FinishedTask::new(AllocHandle::empty(), Self::empty)))
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
    function: fn(&mut Game, AllocHandle),
}

impl FinishedTask {
    pub fn new(handle: AllocHandle, function: fn(&mut Game, AllocHandle)) -> Self {
        return FinishedTask {
            handle,
            function,
        };
    }

    pub fn call(self, game: &mut Game) {
        (self.function)(game, self.handle);
    }
}