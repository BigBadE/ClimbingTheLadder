use game::mods::mods::GameMod;
use game::resources::content_pack::ContentPack;
use game::resources::ContentLoader;

pub struct DesktopLoader {

}

impl ContentLoader for DesktopLoader {
    fn load_main_content(&self) -> ContentPack {
        todo!()
    }

    fn load_mod_content(&self, game_mod: GameMod) -> ContentPack {
        todo!()
    }
}

impl DesktopLoader {
    pub fn new() -> Self {
        return Self {

        }
    }
}