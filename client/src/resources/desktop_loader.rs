use std::collections::HashMap;
use std::{fs, path};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::Error;
use image::{ImageFormat, RgbaImage};
use json::JsonValue;
use tokio::runtime::Handle;
use tokio::task::{JoinHandle, JoinSet};
use game::error;
use game::language::language::LanguagePack;
use game::rendering::{AssetType, GameTexture};
use game::rendering::mesh::Mesh;
use crate::resources::content_pack::ContentPack;

#[derive(Clone)]
pub struct DesktopLoader {
    root: PathBuf,
}

impl ContentPack for DesktopLoader {
    fn types(&self) -> Vec<PathBuf> {
        let path = self.root.join("types");

        if !path.exists() {
            return Vec::new();
        }

        let mut loading = Vec::new();
        return match DesktopLoader::find_files(path, &mut loading, vec!()) {
            Ok(_) => loading,
            Err(error) => {
                error!("Error loading JSON types: {}", error);
                return Vec::new();
            }
        };
    }

    fn get_relative(&self, subpath: &str, path: &PathBuf) -> String {
        return path.to_str().unwrap().replace(&self.root.to_str().unwrap().to_string(), "")
            .replace(path::MAIN_SEPARATOR, "/")
            .replace(&("/".to_string() + subpath), "")
            .split('.').nth(0).unwrap().to_string();
    }

    fn assets(&self, handle: &Handle, load_first: bool) -> JoinHandle<Vec<AssetType>> {
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
        let loading;
        if load_first {
            loading = DesktopLoader::find_files(self.root.join("assets/textures/load_first"), &mut temp, vec!());
        } else {
            loading = DesktopLoader::find_files(self.root.join("assets/textures"), &mut temp, vec!("load_first"));
        }

        match loading {
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

    fn shaders(&self, early: bool) -> Vec<PathBuf> {
        let shaders;
        let mut loading = Vec::new();
        if early {
            shaders = DesktopLoader::find_files(self.root.join("shaders/load_first"), &mut loading, Vec::new());
        } else {
            shaders = DesktopLoader::find_files(self.root.join("shaders"), &mut loading, vec!("load_first"));
        }
        return match shaders {
            Ok(_) => loading,
            Err(error) => {
                error!("Error loading shaders: {}", error);
                return Vec::new();
            }
        };
    }

    fn clone_boxed(&self) -> Box<dyn ContentPack> {
        return Box::new(self.clone());
    }
}

#[derive(Debug)]
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

    fn get_relative_path(base: PathBuf, path: PathBuf) -> String {
        let mut name = path.to_str().unwrap().replace(base.to_str().unwrap(), "")
            .replace(path::MAIN_SEPARATOR, "/").split('.').nth(0).unwrap().to_string();
        name.remove(0);
        return name;
    }

    fn find_files(directory: PathBuf, output: &mut Vec<PathBuf>, ignoring: Vec<&str>) -> Result<(), Error> {
        for file in fs::read_dir(directory.clone())? {
            let file = file?;
            if file.file_type()?.is_file() {
                output.push(file.path());
            } else {
                if ignoring.contains(&file.file_name().to_str().unwrap()) {
                    continue
                }
                DesktopLoader::find_files(file.path(), output, Vec::new())?
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
        DesktopLoader::find_files(directory.clone(), &mut temp, Vec::new())?;
        for file in temp {
            output.push((Self::get_relative_path(directory.clone(), file.clone()),
                         json::parse(String::from_utf8(fs::read(file)?)?.as_str())?));
        }
        return Ok(output);
    }
}