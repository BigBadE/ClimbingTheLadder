use std::{env, fs};
use std::fs::DirEntry;
use std::future::Future;
use anyhow::Error;
use libloading::{Library, Symbol};

use core::mods::mod_trait::ModMain;
use core::mods::mods::{GameMod, ModManifest};

pub async fn load_mods() -> Vec<impl Future<Output=Result<GameMod, Error>>> {
    let mod_folder = env::current_dir().ok().unwrap().join("mods");
    fs::create_dir_all(&mod_folder).unwrap();
    let mut output = Vec::new();
    for mod_folder in fs::read_dir(mod_folder).unwrap() {
        match mod_folder {
            Ok(mod_folder) => output.push(load_mod(mod_folder)),
            Err(error) => println!("Error loading mod:\n{}", error)
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
    let func: Symbol<unsafe extern fn() -> Box<dyn ModMain>> = unsafe { library.get(manifest.main.as_bytes())? };
    return Ok(GameMod::new(manifest, unsafe { func() }));
}