use std::collections::HashMap;
use json::JsonValue;

pub enum LanguagePack {
    Translations(Vec<JsonValue>)
}

pub struct LanguageManager {
    translations: Box<HashMap<String, String>>
}

impl LanguageManager {
    pub fn translate(&self, input: String) -> String {
        return match self.translations.get(&input) {
            Some(value) => value.clone(),
            None => format!("<strike>{}</strike>", input)
        };
    }

    pub fn load(&mut self, packs: Vec<LanguagePack>) {
        todo!()
    }
}