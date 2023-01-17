use std::collections::HashMap;
use std::path::PathBuf;
use std::string::String;
use anyhow::Error;
use json::JsonValue;
use macros::JsonResource;
use crate::mods::mod_trait::ModMain;

//A loaded mod
pub struct GameMod {
    manifest: ModManifest,
    content_root: PathBuf,
    main: Box<dyn ModMain + Send>
}

impl GameMod {
    pub fn new(manifest: ModManifest, content_root: PathBuf, main: Box<dyn ModMain + Send>) -> Self {
        return Self {
            manifest,
            content_root,
            main
        }
    }
}

#[derive(JsonResource)]
pub struct ModManifest {
    pub name: String,
    pub main: String,
    #[ignore_field]
    pub platforms: HashMap<String, String>
}

impl ModManifest {
    fn new() -> Self {
        return Self {
            name: String::new(),
            main: String::new(),
            platforms: HashMap::new()
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
        return Ok(returning)
    }
}