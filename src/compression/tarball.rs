use error_chain::error_chain;
use flate2::read::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::fs::File;
use std::path::PathBuf;
use tar::Archive;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        StripPrefix(std::path::StripPrefixError);
    }
}

pub fn unpack() -> Result<()> {
    let path = "archive.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;
    Ok(())
}

pub fn pack_dir() -> Result<()> {
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("target\\debug", "src\\bin")?;
    Ok(())
}

pub fn unpack_dir() -> Result<()> {
    let file = File::open("archive.tar.gz")?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = "target/debug";

    println!("Extracted the following files:");
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?.strip_prefix(prefix)?.to_owned();
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    Ok(())
}
