use crate::mods::mods::GameMod;
use crate::resources::content_pack::ContentPack;

pub mod content_pack;
pub mod resource_manager;

pub trait ContentLoader {
    fn load_main_content(&self) -> ContentPack;

    fn load_mod_content(&self, game_mod: GameMod) -> ContentPack;
}