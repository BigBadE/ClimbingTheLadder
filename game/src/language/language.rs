use std::collections::HashMap;
use std::sync::RwLock;
use json::JsonValue;
use crate::error;

pub enum LanguagePack {
    Translations(Vec<JsonValue>)
}

pub struct LanguageManager {
    translations: Box<HashMap<String, String>>
}

pub static LANGUAGE_MANAGER: RwLock<LanguageManager> = RwLock::new(LanguageManager::new());

impl LanguageManager {
    fn new() -> Self {
        return Self {
            translations: Box::new(HashMap::new())
        }
    }

    pub fn translate(&self, input: &String) -> String {
        return match self.translations.get(input) {
            Some(value) => value.clone(),
            None => format!("<strike>{}</strike>", input)
        };
    }

    pub fn load(&mut self, packs: Vec<LanguagePack>) {
        for pack in packs {
            match pack {
                LanguagePack::Translations(translations) => {
                    for value in translations {
                        for (key, value) in value.entries() {
                            self.translations.insert(key.to_string(), value.to_string());
                        }
                    }
                }
            }
        }
    }
}