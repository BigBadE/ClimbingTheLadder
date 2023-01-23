use crate::world::entities::entity::Entity;

pub enum WorldInput {
    Update,
    //Tells the update thread to spawn the entity
    SpawnEntity(Entity),
    //Catch-all for issues
    Error
}