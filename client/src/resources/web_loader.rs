#[cfg(target_arch = "wasm32")]
use include_dir::Dir;
use game::mods::mods::GameMod;
use game::resources::content_pack::ContentPack;
use game::resources::ContentLoader;

pub struct WebLoader {

}

impl ContentLoader for WebLoader {
    fn load_main_content(&self) -> ContentPack {
        todo!()
    }

    fn load_mod_content(&self, game_mod: GameMod) -> ContentPack {
        todo!()
    }
}

impl WebLoader {
    #[cfg(target_arch = "wasm32")]
    pub fn new(resources: Dir<'_>) -> Self {
        return Self {

        }
    }
}