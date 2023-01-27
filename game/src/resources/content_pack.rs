use std::path::PathBuf;
use crate::language::language::LanguagePack;

pub trait ContentPack: Send {
    fn shaders(&self) -> Vec<(String, String)>;

    //This is ignored for mods because they're not loaded yet. Only used for UI shaders.
    fn load_first_shaders(&self) -> Vec<(String, String)>;

    fn types(&self) -> Vec<PathBuf>;

    fn language(&self) -> Vec<LanguagePack>;
}