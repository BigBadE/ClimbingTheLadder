use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use crate::RuntimeFactory;

pub struct World {
    sender: Sender<()>
}

impl World {
    pub fn new(runtime_factory: Box<dyn RuntimeFactory>) -> Self {
        let (sender, receiver): (Sender<()>, Receiver<()>) = mpsc::channel();

        runtime_factory.spawn().spawn(Self::update_async(receiver));

        return Self {
            sender
        }
    }

    pub fn update(&mut self) {

    }

    pub async fn update_async(receiver: Receiver<()>) {

    }
}