use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender};
use crate::{Renderer, TaskManager, ThingRegister};
use crate::world::attachments::WorldAttachment;
use crate::world::channeling::WorldInput;
use crate::world::entities::entity::Entity;
use crate::world::rooms::room::Room;

pub struct World {
    //Errors on senders can safely be unwrapped because if the receiver is dropped
    //we're already extremely fucked
    input_sender: Sender<WorldInput>
}

pub struct WorldData {
    rooms: Vec<Room>,
    input_receiver: Receiver<WorldInput>,
    #[cfg(feature = "renderer")]
    renderer: Arc<dyn Renderer>
}

impl World {
    pub fn new(task_manager: &TaskManager, #[cfg(feature = "renderer")] renderer: Arc<dyn Renderer>,
               found_attachments: &Box<dyn ThingRegister>) -> Self {
        let (input_sender, input_receiver): (Sender<WorldInput>, Receiver<WorldInput>) = mpsc::channel();

        let temp = Self {
            input_sender
        };

        let mut attachments = Vec::new();
        for attachment in found_attachments.registered() {
            attachments.push(attachment.deref());
        }

        #[cfg(feature = "renderer")]
        task_manager.get_runtime(false).spawn(Self::update_async(
            WorldData::new(input_receiver, renderer), attachments));
        #[cfg(not(feature = "renderer"))]
        task_manager.get_runtime(false).spawn(Self::update_async(
            WorldData::new(input_receiver), attachments));
        return temp;
    }

    pub fn generate(world_data: &mut WorldData) {
        #[cfg(not(feature = "renderer"))]
        world_data.rooms.push(Room::new());
        #[cfg(feature = "renderer")]
        world_data.rooms.push(Room::new(world_data.renderer.clone()));
    }

    pub fn update(&mut self) {
        self.input_sender.send(WorldInput::Update).unwrap();
    }

    pub fn spawn(&mut self, entity: Entity) {
        self.input_sender.send(WorldInput::SpawnEntity(entity)).unwrap();
    }

    pub async fn update_async(mut world_data: WorldData, mut attachments: Vec<Box<dyn WorldAttachment>>) {
        Self::generate(&mut world_data);
        loop {
            match world_data.input_receiver.recv() {
                Ok(output) => match output {
                    WorldInput::Update => {}
                    WorldInput::SpawnEntity(entity) => entity.spawn(&mut world_data),
                    //Prevent bugs causing turbo updates
                    _ => continue
                }
                Err(_error) => return
            }

            for room in &mut world_data.rooms {
                room.update();
            }

            for attachment in &mut attachments {
                attachment.update(&mut world_data);
            }
        }
    }
}

impl WorldData {
    pub fn new(input_receiver: Receiver<WorldInput>, #[cfg(feature = "renderer")] renderer: Arc<dyn Renderer>) -> Self {
        return Self {
            rooms: Vec::new(),
            input_receiver,
            #[cfg(feature = "renderer")]
            renderer
        };
    }
}