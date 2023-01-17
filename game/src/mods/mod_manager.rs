use anyhow::Error;
use tokio::task::JoinSet;
use crate::{error, GameMod};

pub struct ModManager {
    loading: JoinSet<Result<GameMod, Error>>,
    loaded_mods: Vec<GameMod>
}

impl ModManager {
    pub fn new(loading: JoinSet<Result<GameMod, Error>>) -> Self {
        return Self {
            loading,
            loaded_mods: Vec::new()
        }
    }

    pub async fn get_mods(&mut self) -> &Vec<GameMod> {
        //Block if mods haven't loaded yet
        if self.loading.is_empty() {
            return &self.loaded_mods;
        }

        while let Some(found) = self.loading.join_next().await {
            match found {
                Ok(found) => match found {
                    Ok(found) => self.loaded_mods.push(found),
                    Err(error) => error!("Error loading mod: {}", error)
                },
                Err(error) => error!("Panic with mod loading thread: {}", error)
            }
        }

        return &self.loaded_mods;
    }
}