use std::ops::Deref;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use anyhow::Error;
use tokio::runtime::Handle;
use crate::{Renderer, ThingRegister};
use crate::resources::resource_manager::ResourceManager;
use crate::world::attachments::WorldAttachment;
use crate::world::channeling::WorldInput;
use crate::world::entities::entity::Entity;
use crate::world::rooms::room::Room;

pub struct World {
    //Errors on senders can safely be unwrapped because if the receiver is dropped
    //we're already extremely fucked
    input_sender: Sender<WorldInput>,
}

pub struct WorldData {
    rooms: Vec<Room>,
    input_receiver: Receiver<WorldInput>,
    #[cfg(feature = "renderer")]
    renderer: Arc<dyn Renderer>,
}

impl World {
    pub fn new(runtime: &Handle, #[cfg(feature = "renderer")] renderer: Arc<dyn Renderer>,
               resources: Arc<Mutex<ResourceManager>>, found_attachments: Arc<dyn ThingRegister>) -> Self {
        let (input_sender, input_receiver): (Sender<WorldInput>, Receiver<WorldInput>) = mpsc::channel();

        let temp = Self {
            input_sender
        };

        let mut attachments = Vec::new();
        for attachment in found_attachments.registered() {
            attachments.push(attachment.deref());
        }

        #[cfg(feature = "renderer")]
        runtime.spawn(Self::update_async(
            WorldData::new(input_receiver, renderer), attachments, resources));
        #[cfg(not(feature = "renderer"))]
        runtime.spawn(Self::update_async(WorldData::new(input_receiver), attachments));
        return temp;
    }

    pub fn generate(world_data: &mut WorldData, resources: Arc<Mutex<ResourceManager>>) {
        #[cfg(not(feature = "renderer"))]
        world_data.rooms.push(Room::new(resources.lock().unwrap().deref()));
        #[cfg(feature = "renderer")]
        world_data.rooms.push(Room::new(resources.lock().unwrap().deref(), world_data.renderer.clone()));
    }

    pub fn update(&mut self) -> Result<(), Error> {
        self.input_sender.send(WorldInput::Update)?;
        return Ok(());
    }

    pub fn spawn(&mut self, entity: Entity) -> Result<(), Error> {
        self.input_sender.send(WorldInput::SpawnEntity(entity))?;
        return Ok(());
    }

    pub async fn update_async(mut world_data: WorldData, mut attachments: Vec<Box<dyn WorldAttachment>>,
                              resources: Arc<Mutex<ResourceManager>>) {
        Self::generate(&mut world_data, resources);
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
            renderer,
        };
    }
}