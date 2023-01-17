use std::env;
use std::os::windows::fs::symlink_dir;
use std::path::Path;

#[cfg(target_arch = "wasm32")]
pub fn main() {

}

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    let input = Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str()).join("../resources");
    let output = Path::new(env::var("OUT_DIR").unwrap().as_str()).join("resources");
    if !output.exists() {
        if symlink_dir(input, &output).is_err() {
            panic!("Failed to create symlink in {}.\nRead the Building section of the README or create the symlink yourself.", output.display())
        }
    }
}