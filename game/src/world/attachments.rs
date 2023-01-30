use crate::world::world::WorldData;

pub trait WorldAttachment: Send {
    fn update(&mut self, world: &mut WorldData);
}