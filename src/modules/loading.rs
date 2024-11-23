/*
Coutils by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Bytes"
/// structure to handle the
/// data a response returns.
use bytes::Bytes;

/// Importing the "get"
/// function from the
/// "reqwest" crate to
/// fetch assets from the
/// interwebs.
use reqwest::get;

/// Importing Rust's 
/// "File" struct
/// from the "fs" module.
use std::fs::File;

/// Importing the "Write"
/// trait to write to files.
use std::io::Write;

/// Importing the "Response"
/// structure for explicit typing.
use reqwest::Response;

/// Importing this crate's error
/// structure.
use super::error::CoutilsError;

/// Attempts to download files from the internet and store the retrieved
/// bytes in the file at the given path. If this operation fails, an error
/// is returned.
pub async fn download_file(url: &str, path: &str) -> Result<(), CoutilsError>{
    let dl: Response = match get(url).await{
        Ok(dl) => dl,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    let file_bytes: Bytes = match dl.bytes().await{
        Ok(file_bytes) => file_bytes,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    let mut target_file: File = match File::create(path){
        Ok(target_file) => target_file,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    let write_op = match target_file.write_all(&file_bytes){
        Ok(write_op) => write_op,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    Ok(write_op)
}