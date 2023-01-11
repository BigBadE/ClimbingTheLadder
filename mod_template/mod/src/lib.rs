pub struct ModTemplate {

}

impl ModMain for ModTemplate {

}

impl ModTemplate {
    pub fn new() -> Self {
        return Self {

        };
    }

    //Must be [no_mangle] or Rust will rename it to something else.
    #[no_mangle]
    pub fn mod_template_main() -> Box<dyn ModMain> {
        return Box::new(ModMain::new());
    }
}