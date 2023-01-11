use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let profile = env::var("PROFILE").unwrap_or("release".to_string());
    let targets;
    if profile == "release" {
        targets = vec!(format!("{}-{}", env::consts::ARCH, env::consts::OS).as_str());
    } else {
        targets = vec!("x86_64-unknown-none", "arm-unknown-none", "aarch64-unknown-none");
    }
}

fn build(name: &str, path: &str, target: &str, profile: &str, out_dir: &Path) -> PathBuf {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let mut cmd = Command::new(cargo);
    cmd.arg("install").arg(name.clone());
    cmd.arg("--path").arg(path.clone());
    cmd.arg("--locked");
    cmd.arg("--target").arg(format!("{}/{}", path, target));
    cmd.arg("--root").arg(out_dir);
    cmd.arg("--profile").arg(profile);
    cmd.env_remove("RUSTFLAGS");
    cmd.env_remove("CARGO_ENCODED_RUSTFLAGS");
    let status = cmd
        .status()
        .expect(format!("failed to run cargo install for {}", name).as_str());
    if status.success() {
        let path = out_dir.join("bin").join(name);
        assert!(
            path.exists(),
            "{} library does not exist after building", name
        );
        path
    } else {
        panic!("failed to build {}", name);
    }
}