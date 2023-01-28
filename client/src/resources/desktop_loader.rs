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

    fn load_first_shaders(&self) -> Vec<(String, String)> {
        return match DesktopLoader::load_text(self.root.join("shaders/load_first")) {
            Ok(result) => result,
            Err(error) => {
                error!("Error loading load-first shaders: {}", error);
                return Vec::new();
            }
        }
    }

    fn types(&self) -> Vec<PathBuf> {
        let path = self.root.join("types");

        if !path.exists() {
            return Vec::new();
        }

        let mut loading = Vec::new();
        return match DesktopLoader::find_files(path, &mut loading) {
            Ok(_) => loading,
            Err(error) => {
                error!("Error loading JSON types: {}", error);
                return Vec::new();
            }
        }
    }

    fn language(&self) -> Vec<LanguagePack> {
        let mut output = Vec::new();
        match DesktopLoader::load_json(self.root.join("language/translations")) {
            Ok(value) => output.push(LanguagePack::Translations(value)),
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
            output.push((file.file_name().into_string()?.split('.').nth(0).unwrap().to_string(),
                         String::from_utf8(fs::read(file.path())?)?));
        }
        return Ok(output);
    }

    fn find_files(directory: PathBuf, output: &mut Vec<PathBuf>) -> Result<(), Error> {
        for file in fs::read_dir(directory)? {
            let file = file?;
            if file.file_type()?.is_file() {
                output.push(file.path());
            } else {
                DesktopLoader::find_files(file.path(), output)?
            }
        }
        return Ok(());
    }

    fn load_json(directory: PathBuf) -> Result<Vec<JsonValue>, Error> {
        if !directory.exists() {
            return Ok(Vec::new());
        }

        let mut output = Vec::new();
        let mut temp = Vec::new();
        DesktopLoader::find_files(directory, &mut temp)?;
        for file in temp {
            output.push(json::parse(String::from_utf8(fs::read(file)?)?.as_str())?)
        }
        return Ok(output);
    }
}