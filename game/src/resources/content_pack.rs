use json::JsonValue;
use crate::language::language::{LanguageManager, LanguagePack};

pub trait ContentPack: Send {
    fn shaders(&self) -> Vec<(String, String)>;

    fn types(&self) -> Vec<JsonValue>;

    fn language(&self) -> Vec<LanguagePack>;
}