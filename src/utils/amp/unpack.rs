use std::{fs::File, io::BufReader, path::Path};

use tar::Archive;




pub fn unpack_amp<P: AsRef<Path>>(archive_path: P, dist: P) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(archive_path)?;
    let bufferd_file = BufReader::new(file);
    let decoder = zstd::Decoder::new(bufferd_file)?;

    let mut archive = Archive::new(decoder);

    archive.unpack(dist)?;

    Ok(())
}
