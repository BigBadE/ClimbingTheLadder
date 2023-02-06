use std::collections::HashMap;
use std::string::String;
use macros::JsonLoadable;
use crate::mods::mod_trait::ModMain;

//A loaded mod
pub struct GameMod {
    _manifest: ModManifest,
    _main: Box<dyn ModMain + Send>
}

impl GameMod {
    pub fn new(manifest: ModManifest, main: Box<dyn ModMain + Send>) -> Self {
        return Self {
            _manifest: manifest,
            _main: main
        }
    }
}

#[derive(JsonLoadable, Default)]
pub struct ModManifest {
    #[require_field]
    pub name: String,
    #[require_field]
    pub main: String,
    #[require_field]
    pub platforms: HashMap<String, String>,
    pub hard_dependencies: Vec<String>,
    pub soft_dependencies: Vec<String>
}