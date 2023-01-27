use std::any::TypeId;
use std::collections::HashMap;
use std::future::Future;
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use anyhow::Error;
use json::object::Object;
use tokio::task::JoinHandle;
use crate::ResourceManager;
use crate::resources::resource_manager::NamedType;

pub struct ResourceLoader {
    total_tasks: u32,
    sleeping: u32,
    deadlocked: bool,
    reference: Arc<Mutex<ResourceManager>>,
    wakers: HashMap<String, Vec<Waker>>
}

impl ResourceLoader {
    pub fn new(reference: Arc<Mutex<ResourceManager>>) -> Self {
        return Self {
            total_tasks: 0,
            sleeping: 0,
            deadlocked: false,
            reference,
            wakers: HashMap::new()
        }
    }
    
    pub fn spawn(reference: Arc<Mutex<ResourceLoader>>, object: Object) -> impl Future<Output=Result<(TypeId, Box<dyn NamedType>), Error>>{
        return ResourceLoadTask::new(object, reference);
    }
    
    fn deadlock(&mut self) {
        self.deadlocked = true;
        for wakers in self.wakers.values() {
            for waker in wakers {
                waker.wake()
            }
        }
    }
}

pub struct ResourceLoadTask {
    object: Object,
    loader: Arc<Mutex<ResourceLoader>>
}

impl ResourceLoadTask {
    pub fn new(object: Object, loader: Arc<Mutex<ResourceLoader>>) -> Self {
        return Self {
            object,
            loader
        }
    }
}

impl Future for ResourceLoadTask {
    type Output = Result<(TypeId, Box<dyn NamedType>), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let obj_type = match self.object.get("$type") {
            Some(name) => name,
            None => return Poll::Ready(Err(Error::msg(
                format!("No name for parent type in JSON\n{:?}", self.object))))
        }.dump();
        let obj_type = obj_type.as_str();

        let mut loader = self.loader.lock().unwrap();
        let mut manager = loader.reference.lock().unwrap();
        
        let creator = match manager.instantiators.get(obj_type) {
            Some(instantiator) => match instantiator(manager.deref_mut(), &self.object) {
                Ok(value) => match value {
                    Ok(creator) => creator,
                    Err(blocked) => {
                        if loader.deadlocked {
                            return Poll::Ready(Err(Error::msg(format!(
                                "Failed to find NamedType {} for JSON:\n{:?}", blocked, self.object))))
                        }
                        loader.sleeping += 1;
                        if loader.sleeping == loader.total_tasks {
                            loader.deadlock();
                        }
                        match loader.wakers.get_mut(&blocked) {
                            Some(vec) => vec.push(cx.waker().clone()),
                            None => {
                                loader.wakers.insert(blocked, vec!(cx.waker().clone()));
                            }
                        }
                        return Poll::Pending;
                    }
                },
                Err(error) => return Poll::Ready(Err(error))
            },
            None => {
                return Poll::Ready(Err(Error::msg(
                    format!("No parent type ($type) in JSON\n{:?}", self.object))));
            }
        };

        let name = match self.object.get("$name") {
            Some(name) => name.dump(),
            None => return Poll::Ready(Err(Error::msg(
                format!("No name ($name( in JSON\n{:?}", self.object))))
        };
        
        //Reduce the tasks and wake everyone up.
        loader.total_tasks -= 1;
        match loader.wakers.get(&name) {
            Some(wakers) => for waker in wakers {
                waker.wake()
            },
            None => {}
        }         
        return Poll::Ready(Ok(creator))
    }
}