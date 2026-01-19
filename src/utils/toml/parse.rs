use std::path::Path;
use crate::err;


pub fn parse_toml<T: serde::de::DeserializeOwned, P: AsRef<Path>>(file_path: P) -> T {
    let toml_str = std::fs::read_to_string(file_path.as_ref()).unwrap_or_else(|e| {
        err!("Failed to read TOML file {}: {}", file_path.as_ref().display(), e);
        std::process::exit(1);
    });

    toml::from_str(&toml_str).unwrap_or_else(|e| {
        err!("Failed to parse TOML file {}: {}", file_path.as_ref().display(), e);
        std::process::exit(1);
    })
}





