use std::{fs, io::Read, path::Path, process::exit};

use rkyv::rancor;

use crate::{err, utils::pkgdb::structs};


pub fn read_db<P: AsRef<Path>>(file: P) -> structs::PackageDB {
    let file_bytes = fs::File::open(file).unwrap_or_else(|e| {
        err!("Failed to read packages database: {}", e);
        exit(1);
    });
    let mut buffer = Vec::new();
    file_bytes.read_to_end(&mut buffer).unwrap();

    let archived = rkyv::access::<_, rancor::Error>(&buffer).unwrap_or_else(|e| {
        err!("Failed to access archived packages database: {}", e);
        exit(1);
    });


    

}

