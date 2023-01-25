use std::{env, fs};
use std::fs::DirEntry;
use std::future::Future;
use anyhow::Error;
use libloading::{Library, Symbol};
use log::error;
use tokio::runtime::Runtime;
use tokio::task::{JoinHandle, JoinSet};
use game::mods::mod_trait::ModMain;
use game::mods::mods::{GameMod, ModManifest};
use crate::DesktopLoader;

pub fn load_mods(runtime: &Runtime) -> JoinSet<Result<GameMod, Error>> {
    let mod_folder = env::current_dir().ok().unwrap().join("mods");
    fs::create_dir_all(&mod_folder).unwrap();
    let mut output = JoinSet::new();
    for mod_folder in fs::read_dir(mod_folder).unwrap() {
        match mod_folder {
            Ok(mod_folder) => {
                output.spawn_on(load_mod(mod_folder), runtime.handle());
            },
            Err(error) => error!("Error opening mod folder:\n{}", error)
        }
    }
    return output;
}

async fn load_mod(mod_folder: DirEntry) -> Result<GameMod, Error> {
    let manifest = mod_folder.path().join("manifest.json");
    if !manifest.exists() {
        return Err(Error::msg(format!("Mod {} has no manifest, ignoring", mod_folder.file_name().to_str().unwrap())));
    }
    let manifest = json::from(fs::read(manifest).unwrap());
    let manifest = ModManifest::load(&manifest)?;
    let target = mod_folder.path().join("assemblies")
        .join(format!("{}.rlib", env::consts::ARCH));
    if !target.exists() {
        return Err(Error::msg(format!("Mod {} doesn't support arch {} (no rlib found)",
                                      manifest.name, env::consts::ARCH)))
    }
    let library = match unsafe { Library::new(target) } {
        Ok(lib) => lib,
        Err(error) => {
            return Err(Error::msg(format!("Failed to load mod {}", manifest.name)).context(error));
        }
    };
    let func: Symbol<unsafe extern fn() -> Box<dyn ModMain + Send>> = unsafe { library.get(manifest.main.as_bytes())? };
    return Ok(GameMod::new(manifest, Box::new(DesktopLoader::new(mod_folder.path())), unsafe { func() }));
}