#[cfg(target_arch = "wasm32")]
use include_dir::Dir;
use core::resources::ContentLoader;

pub struct WebLoader {

}

impl ContentLoader for WebLoader {

}

impl WebLoader {
    #[cfg(target_arch = "wasm32")]
    pub fn new(resources: Dir<'_>) -> Self {
        return Self {

        }
    }
}