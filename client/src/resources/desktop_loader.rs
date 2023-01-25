use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use json::JsonValue;
use game::language::language::{LanguageManager, LanguagePack};
use game::mods::mods::GameMod;
use game::resources::content_pack::ContentPack;
use game::resources::ContentLoader;

pub struct DesktopLoader {
    root: PathBuf
}

impl ContentPack for DesktopLoader {
    fn shaders(&self) -> Vec<String> {
        let mut output = Vec::new();
        for file in fs::read_dir(self.root.join("shaders")).unwrap() {
            let file = file.unwrap();
            output.push(String::from_utf8(fs::read(file).unwrap()).unwrap());
        }
        return output;
    }

    fn types(&self) -> Vec<JsonValue> {
        return self.load_json(self.root.join("types"));
    }

    fn language(&self) -> Vec<LanguagePack> {
        let mut output = Vec::new();
        output.push(LanguagePack::Translations(self.load_json(self.root.join("language/translations"))));
        return output;
    }
}

impl DesktopLoader {
    pub fn new(root: PathBuf) -> Self {
        return Self {
            root
        }
    }

    fn load_json(directory: PathBuf) -> Vec<JsonValue> {
        if !directory.exists() {
            return Vec::new();
        }
        let mut output = Vec::new();
        for file in fs::read_dir(directory).unwrap() {
            let file = file.unwrap();
            let file = File::open(file.path()).unwrap();
            output.push(json::parse(
                String::from_utf8(fs::read(file).unwrap()).unwrap().as_str()).unwrap());
        }
        return output;
    }
}