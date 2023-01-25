use std::fs;
use std::fs::File;
use std::path::PathBuf;
use json::JsonValue;
use game::language::language::LanguagePack;
use game::resources::content_pack::ContentPack;

pub struct DesktopLoader {
    root: PathBuf
}

impl ContentPack for DesktopLoader {
    fn shaders(&self) -> Vec<(String, String)> {
        let mut output = Vec::new();
        for file in fs::read_dir(self.root.join("shaders")).unwrap() {
            let file = file.unwrap();
            output.push((file.file_name().into_string().unwrap().split('.').nth(0).unwrap().to_string(),
                         String::from_utf8(fs::read(file.path()).unwrap()).unwrap()));
        }
        return output;
    }

    fn types(&self) -> Vec<JsonValue> {
        return DesktopLoader::load_json(self.root.join("types"));
    }

    fn language(&self) -> Vec<LanguagePack> {
        let mut output = Vec::new();
        output.push(LanguagePack::Translations(DesktopLoader::load_json(self.root.join("language/translations"))));
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
            output.push(json::parse(
                String::from_utf8(fs::read(file.path()).unwrap()).unwrap().as_str()).unwrap());
        }
        return output;
    }
}