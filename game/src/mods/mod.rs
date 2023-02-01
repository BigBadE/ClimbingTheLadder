use anyhow::Error;
use tokio::runtime::Handle;
use tokio::task::JoinSet;
use crate::mods::mods::GameMod;

pub mod mod_manager;
pub mod mod_trait;
pub mod mods;

pub trait ModProvider {
    fn get_mods(&self, runtime: &Handle) -> JoinSet<Result<GameMod, Error>>;
}