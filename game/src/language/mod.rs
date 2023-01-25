pub mod language;

trait Translatable {
    fn translate(&self) -> String;
}

impl Translatable for String {
    fn translate(&self) -> String {
        return language::LANGUAGE_MANAGER.read().unwrap().translate(self);
    }
}