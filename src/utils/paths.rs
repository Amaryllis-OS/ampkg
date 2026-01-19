use std::{fs, path::{Path, PathBuf}};

use crate::err;


pub const CONFIG_DIR: &str = "/etc/ampkg";
pub const CACHE_DIR: &str = "/var/cache/ampkg";
pub const LOG_DIR: &str = "/var/log/ampkg";
pub const PUBLIC_KEYS_DIR: &str = "/usr/share/ampkg/keys";
pub const PKGINSTALL_DIR: &str = "/tmp/ampkg/pkginstall";


pub fn get_pkginstall_dir() -> PathBuf {
    let dir = Path::new(PKGINSTALL_DIR);

    fs::create_dir_all(dir).unwrap_or_else(|e| {
        err!("Failed to create pkginstall directory {}: {}", dir.display(), e);
        std::process::exit(1);
    });

    dir.to_path_buf()
}

pub fn get_public_keys() -> Vec<PathBuf> {
    let mut keys = Vec::new();
    let dir = Path::new(PUBLIC_KEYS_DIR);

    fs::create_dir_all(dir).unwrap_or_else(|e| {
        err!("Failed to create public keys directory {}: {}", dir.display(), e);
        std::process::exit(1);
    });

    if dir.exists() && dir.is_dir() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("pub") && path.is_file() {
                keys.push(path);
            }
        }
    }

    keys
}