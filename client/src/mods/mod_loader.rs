use std::{env, fs};
use std::fs::DirEntry;
use std::future::Future;
use std::process::Command;
use anyhow::Error;
use libloading::Library;

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
    let manifest = match ModManifest::load(&manifest) {
        Ok(manifest) => manifest,
        Err(error) => return Err(error)
    };
    if !manifest.platforms.contains_key(env::consts::OS + ":" + env::consts::ARCH) {
        
    }
    let library = match Library::new() {
        Ok(lib) => lib,
        Err(error) => {
            return Err(Error::msg(format!("Failed to load mod {}", manifest.name)).context(error));
        }
    }
    return Some(GameMod::new(manifest));
}