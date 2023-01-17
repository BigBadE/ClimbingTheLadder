use game::mods::mod_trait::ModMain;

use static_assertions::assert_impl_all;

pub struct ModTemplate {

}

// This type must be sync!
assert_impl_all!(ModTemplate: Sync);

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
        return Box::new(ModTemplate::new());
    }
}