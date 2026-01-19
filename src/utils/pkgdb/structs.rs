use rkyv::{Archive, Deserialize, Serialize};


#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub libc: String,
    pub architecture: String,
    pub description: String,
}


#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct PackageDB {
    pub packages: Vec<Package>,
}

