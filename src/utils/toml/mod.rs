pub mod parse;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Control {
    pub package: Package,
    pub dependencies: Dependencies,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub libc: String,
    pub architecture: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependencies {
    pub runtime: Vec<String>,
}
