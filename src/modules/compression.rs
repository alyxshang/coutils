/*
Coutils by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// "Archive" structure
/// from the "tar" crate.
use tar::Archive;

/// Importing Rust's 
/// "File" struct
/// from the "fs" module.
use std::fs::File;

/// Importing the "Compression"
/// structure from the "flate2"
/// crate to specify the level 
/// of compression.
use flate2::Compression;

/// Importing the "GzDecoder"
/// structure from the "flate2"
/// crate to decode Gzip-encoded
/// files.
use flate2::read::GzDecoder;

/// Importing the "GzEncoder"
/// structure from the "flate2"
/// crate to encode Gzip
/// files.
use flate2::write::GzEncoder;

/// Importing this crate's error
/// structure.
use super::error::CoutilsError;

/// Attempts to extract the given tarball into the given target path. If this
/// operation fails, an error is returned.
pub fn extract_tarball(tarball: &str, target_path: &str) -> Result<(), CoutilsError> {
    let tar_gz = match File::open(tarball){
        Ok(tar_gz) => tar_gz,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    let res = match archive.unpack(target_path){
        Ok(res) => res,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    Ok(res)
}

/// Attempts to create a tarball with provided name from the provided directory.
/// If this operation fails, an error is returned.
pub fn create_tarball(directory: &str, target_archive: &str) -> Result<(), CoutilsError> {
    let tar_gz = match File::create(target_archive){
        Ok(tar_gz) => tar_gz,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    let res = match tar.append_dir_all("", directory){
        Ok(res) => res,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    Ok(res)
}