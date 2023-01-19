use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use crate::RuntimeFactory;
use crate::world::channeling::{WorldInput, WorldOutput};
use crate::world::rooms::room::Room;

pub struct World {
    input_sender: Sender<WorldInput>,
    output_receiver: Receiver<WorldOutput>
}

impl World {
    pub fn new(runtime_factory: Box<dyn RuntimeFactory>) -> Self {
        let (input_sender, input_receiver): (Sender<WorldInput>, Receiver<WorldInput>) = mpsc::channel();
        let (output_sender, output_receiver): (Sender<WorldOutput>, Receiver<WorldOutput>) = mpsc::channel();

        let temp = Self {
            input_sender,
            output_receiver
        };
        runtime_factory.spawn().spawn(
            Self::update_async(temp.generate(), input_receiver, output_sender));
        return temp;
    }

    pub fn generate(&self) -> Vec<Room> {
        todo!()
    }

    pub fn update(&mut self) {
        self.input_sender.send(WorldInput::Update).unwrap();
    }

    pub async fn update_async(mut rooms: Vec<Room>, input_receiver: Receiver<WorldInput>, output_sender: Sender<WorldOutput>) {
        loop {
            input_receiver.recv().unwrap();

            for room in &mut rooms {
                room.update();
            }
        }
    }
}