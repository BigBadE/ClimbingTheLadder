use std::collections::HashMap;
use std::string::String;
use anyhow::Error;
use json::JsonValue;
use macros::JsonResource;
use crate::mods::mod_trait::ModMain;
use crate::util::json_util::stringify;

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

#[derive(JsonResource)]
pub struct ModManifest {
    pub name: String,
    pub main: String,
    #[ignore_field]
    pub platforms: HashMap<String, String>,
    #[ignore_field]
    pub hard_dependencies: Vec<String>,
    #[ignore_field]
    pub soft_dependencies: Vec<String>
}

impl ModManifest {
    fn new() -> Self {
        return Self {
            name: String::new(),
            main: String::new(),
            platforms: HashMap::new(),
            hard_dependencies: Vec::new(),
            soft_dependencies: Vec::new()
        }
    }

    pub fn load(manifest: &JsonValue) -> Result<Self, Error> {
        let mut returning = __load_ModManifest(ModManifest::new(), manifest)?;
        for entry in manifest["platforms"].entries() {
            returning.platforms.insert(entry.0.to_string(), match entry.1 {
                JsonValue::String(str) => str.clone(),
                _ => return Err(Error::msg("Unknown type for platform!"))
            }).unwrap();
        }

        returning.hard_dependencies = stringify(&manifest["hard_dependencies"]);
        returning.soft_dependencies = stringify(&manifest["soft_dependencies"]);

        return Ok(returning)
    }
}