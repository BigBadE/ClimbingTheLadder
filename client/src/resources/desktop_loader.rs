use std::fs;
use std::path::PathBuf;
use anyhow::Error;
use json::JsonValue;
use game::error;
use game::language::language::LanguagePack;
use game::resources::content_pack::ContentPack;

#[derive(Clone)]
pub struct DesktopLoader {
    root: PathBuf,
}

impl ContentPack for DesktopLoader {
    fn shaders(&self) -> Vec<(String, String)> {
        return match DesktopLoader::load_text(self.root.join("shaders")) {
            Ok(result) => result,
            Err(error) => {
                error!("Error loading shaders: {}", error);
                return Vec::new();
            }
        }
    }

    fn types(&self) -> Vec<JsonValue> {
        return match DesktopLoader::load_json(self.root.join("types")) {
            Ok(result) => result,
            Err(error) => {
                error!("Error loading JSON types: {}", error);
                return Vec::new();
            }
        }
    }

    fn language(&self) -> Vec<LanguagePack> {
        let mut output = Vec::new();
        match DesktopLoader::load_json(self.root.join("language/translations")) {
            Ok(result) => output.push(LanguagePack::Translations(result)),
            Err(error) => error!("Error loading translations: {}", error)
        }
        return output;
    }
}

impl DesktopLoader {
    pub fn new(root: PathBuf) -> Self {
        return Self {
            root
        };
    }

    fn load_text(directory: PathBuf) -> Result<Vec<(String, String)>, Error> {
        if !directory.exists() {
            return Ok(Vec::new());
        }

        let mut output = Vec::new();
        for file in fs::read_dir(directory)? {
            let file = file?;
            output.push((file.file_name().into_string().ok().unwrap().split('.').nth(0).unwrap().to_string(),
                         String::from_utf8(fs::read(file.path())?)?));
        }
        return Ok(output);
    }

    fn load_json(directory: PathBuf) -> Result<Vec<JsonValue>, Error> {
        if !directory.exists() {
            return Ok(Vec::new());
        }
        let mut output = Vec::new();
        for file in fs::read_dir(directory)? {
            output.push(json::parse(
                String::from_utf8(fs::read(file?.path())?)?.as_str())?);
        }
        return Ok(output);
    }
}