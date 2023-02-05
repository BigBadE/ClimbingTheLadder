use std::collections::HashMap;
use std::{fs, path};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use anyhow::Error;
use image::{ImageFormat, RgbaImage};
use json::JsonValue;
use tokio::runtime::Handle;
use tokio::task::{JoinHandle, JoinSet};
use game::error;
use game::language::language::LanguagePack;
use game::rendering::{AssetType, GameTexture};
use game::rendering::mesh::Mesh;
use game::resources::content_pack::ContentPack;
use game::resources::resource_loader::ResourceLoader;
use game::resources::resource_manager::ResourceManager;
use game::util::task_manager::TaskManager;

#[derive(Clone)]
pub struct DesktopLoader {
    root: PathBuf,
}

impl ContentPack for DesktopLoader {
    fn load(&self, resource_manager: &Arc<Mutex<ResourceManager>>, task_manager: &mut TaskManager) {
        let resource_loader = Arc::new(Mutex::new(ResourceLoader::new(resource_manager.clone())));

        for json in self.types() {
            let loader = task_manager.get_runtime(true).spawn(Self::load_json(json.clone()));
            task_manager.queue(false,
                               Self::load_types(loader, self.get_relative(json), resource_loader.clone(),
                                                task_manager.get_runtime(false).clone()));
        }
    }

    fn early_load(&self, task_manager: &mut TaskManager) {
        todo!()
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
        };
    }

    fn clone_boxed(&self) -> Box<dyn ContentPack> {
        return Box::new(self.clone());
    }

    fn get_relative(&self, path: PathBuf) -> String {
        let mut name = path.to_str().unwrap().replace(self.root.to_str().unwrap(), "")
            .replace(path::MAIN_SEPARATOR, "/").split('.').nth(0).unwrap().to_string();
        name.remove(0);
        return name;
    }
}

impl DesktopLoader {
    fn shaders(&self) -> Vec<(String, String)> {
        return match DesktopLoader::load_text(self.root.join("shaders"), vec!("load_first")) {
            Ok(result) => result,
            Err(error) => {
                error!("Error loading shaders: {}", error);
                return Vec::new();
            }
        };
    }

    fn load_first_shaders(&self) -> Vec<(String, String)> {
        return match DesktopLoader::load_text(self.root.join("shaders/load_first"), vec!()) {
            Ok(result) => result,
            Err(error) => {
                error!("Error loading load-first shaders: {}", error);
                return Vec::new();
            }
        };
    }

    fn assets(&self, handle: &Handle) -> JoinHandle<Vec<AssetType>> {
        let mut output = Vec::new();
        match DesktopLoader::load_json(self.root.join("assets/models")) {
            Ok(value) => {
                let mut loaded = HashMap::new();
                for (file, model) in value {
                    loaded.insert(file, Arc::new(Mesh::load(model)));
                }
                output.push(AssetType::Model(loaded));
            }
            Err(error) => error!("Error loading models: {}", error)
        }

        let mut temp = Vec::new();
        match DesktopLoader::find_files(self.root.join("assets/textures"), &mut temp) {
            Ok(_) => {}
            Err(error) => {
                error!("Error finding textures: {}", error)
            }
        }
        let mut loading = JoinSet::new();
        for texture in temp {
            loading.spawn_on(Self::load_image(self.root.join("assets/textures"), texture), handle);
        }
        return handle.spawn(Self::join_images(loading, output));
    }

    fn language(&self) -> Vec<LanguagePack> {
        let mut output = Vec::new();
        match DesktopLoader::load_json(self.root.join("language/translations")) {
            Ok(found) => {
                let mut values = Vec::new();
                for value in found {
                    values.push(value.1)
                }
                output.push(LanguagePack::Translations(values))
            }
            Err(error) => error!("Error loading translations: {}", error)
        }
        return output;
    }
}

pub struct TextureWrapper {
    texture: RgbaImage,
    name: String,
}

impl Display for TextureWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Texture {}", self.name)
    }
}

impl GameTexture for TextureWrapper {
    fn dimensions(&self) -> (u32, u32) {
        return self.texture.dimensions();
    }

    fn name(&self) -> String {
        return self.name.clone();
    }

    fn data(&self) -> &[u8] {
        return &*self.texture;
    }
}

impl DesktopLoader {
    pub fn new(root: PathBuf) -> Self {
        return Self {
            root
        };
    }

    async fn join_images(mut loading: JoinSet<Result<(String, Arc<dyn GameTexture>), Error>>, mut input: Vec<AssetType>) -> Vec<AssetType> {
        let mut temp = HashMap::new();
        while let Some(found) = loading.join_next().await {
            match found {
                Ok(value) => match value {
                    Ok((name, value)) => {
                        temp.insert(name, value);
                    }
                    Err(error) => error!("Error loading file:\n{}", error)
                },
                Err(error) => error!("Internal error loading file:\n{}", error)
            }
        }
        input.push(AssetType::Texture(temp));
        return input;
    }

    async fn load_image(base: PathBuf, texture: PathBuf) -> Result<(String, Arc<dyn GameTexture>), Error> {
        let loaded = image::load(BufReader::new(File::open(texture.clone())?),
                                 ImageFormat::Png).unwrap().to_rgba8();
        let name = Self::get_relative_path(base, texture);
        return Ok((name.clone(), Arc::new(TextureWrapper {
            texture: loaded,
            name,
        })));
    }

    fn load_text(directory: PathBuf, ignore: Vec<&str>) -> Result<Vec<(String, String)>, Error> {
        if !directory.exists() {
            return Ok(Vec::new());
        }

        let mut output = Vec::new();
        for file in fs::read_dir(directory.clone())? {
            let file = file?;
            let name = Self::get_relative_path(directory.clone(), file.path());
            if ignore.contains(&name.as_str()) {
                continue;
            }
            output.push((name, String::from_utf8(fs::read(file.path())?)?));
        }
        return Ok(output);
    }

    fn get_relative_path(base: PathBuf, path: PathBuf) -> String {
        let mut name = path.to_str().unwrap().replace(base.to_str().unwrap(), "")
            .replace(path::MAIN_SEPARATOR, "/").split('.').nth(0).unwrap().to_string();
        name.remove(0);
        return name;
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

    fn load_json(directory: PathBuf) -> Result<Vec<(String, JsonValue)>, Error> {
        if !directory.exists() {
            return Ok(Vec::new());
        }

        let mut output = Vec::new();
        let mut temp = Vec::new();
        DesktopLoader::find_files(directory.clone(), &mut temp)?;
        for file in temp {
            output.push((Self::get_relative_path(directory.clone(), file.clone()),
                         json::parse(String::from_utf8(fs::read(file)?)?.as_str())?));
        }
        return Ok(output);
    }
}