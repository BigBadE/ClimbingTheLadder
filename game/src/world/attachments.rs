use crate::world::world::WorldData;

pub trait WorldAttachment {
    fn update(&mut self, world: &mut WorldData);
}