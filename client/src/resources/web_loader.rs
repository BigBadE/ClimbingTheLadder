#[cfg(target_arch = "wasm32")]
use include_dir::Dir;
use json::JsonValue;
use game::language::language::LanguagePack;
use game::mods::mods::GameMod;
use game::resources::content_pack::ContentPack;
use game::resources::ContentLoader;

pub struct WebLoader {

}

impl ContentPack for WebLoader {
    fn shaders(&self) -> Vec<String> {
        todo!()
    }

    fn types(&self) -> Vec<JsonValue> {
        todo!()
    }

    fn language(&self) -> Vec<LanguagePack> {
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