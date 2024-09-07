/*
Coutils by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing Rust's 
/// "File" struct
/// from the "fs" module.
use std::fs::File;

/// Importing Rust's "write"
/// function from the "fs"
/// module.
use std::fs::write;

/// Using Rust's standard
/// "Path" API from the
/// "path" module.
use std::path::Path;

/// Rust's file metadata
/// API from the "fs"
/// module.
use std::fs::metadata;

/// Importing the method
/// to copy files
/// from the "fs-extra"
/// crate.
use fs_extra::file::copy;

/// Importing Rust's "remove_file"
/// function from the "fs" module
/// to remove files.
use std::fs::remove_file;

/// Importing the "PartialEq"
/// trait from Rust's "cmp"
/// module.
use std::cmp::PartialEq;

/// Importing the enum
/// that describes types
/// of filesystem entities.
use super::fsentity::Entity;

/// Importing Rust's
/// "read_to_string" function.
use std::fs::read_to_string;

/// We import the "move_file"
/// method from the "fs_extra"
/// crate.
use fs_extra::file::move_file;

/// Importing the enum
/// to store information
/// about filesystem entities.
use super::fsentity::FileEntry;

/// Importing this crate's error
/// structure.
use super::error::CoutilsError;

/// We need this structure to
/// perform copying operations
/// on files.
use fs_extra::file::CopyOptions;

/// Tries to move a file from "src" to "target"
/// and returns a result type depending on whether the
/// operation succeeded or not.
pub fn file_move(src: &str, target: &str) -> Result<(), CoutilsError> {
    let options = CopyOptions::new();
    let move_op = move_file(src, target, &options);
    match move_op {
        Ok(_n) => {},
        Err(e) => {
            return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()));
        }
    }
    return Ok(());
}

/// Checks whether a file exists and
/// returns a boolean to that effect.
pub fn file_is(filename: &str) -> bool {
    return Path::new(filename).exists();
}

/// Tries to create a file and returns
/// a result type depending on whether the
/// operation succeeded or not.
pub fn create_file(filename: &str) -> Result<(), CoutilsError>{
    let new_file = File::create(filename);
    match new_file {
        Ok(_n) => {},
        Err(e) => {
            return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()));
        }
    }
    return Ok(());
}

/// Tries to write to a file and returns
/// a result type depending on whether the
/// operation succeeded or not.
pub fn write_to_file(
    filename: &str, 
    contents: &str
) -> Result<(), CoutilsError> {
    if file_is(filename) == true {
        let write_op = write(filename, contents);
        match write_op {
            Ok(_n) => {},
            Err(e) => {
                return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()));
            }
        }
    }
    else {}
    return Ok(());
}

/// Tries to read a file and return
/// its contents as a string.
pub fn read_file(filename: &str) -> Result<String, CoutilsError> {
    let mut result: String = String::from("");
    if file_is(filename) == true {
        result = match read_to_string(filename) {
            Ok(result) => result,
            Err(e) => {
                return Err::<String, CoutilsError>(CoutilsError::new(&e.to_string()));
            }
        };
    }
    else {}
    Ok(result)
}

/// Checks whether "entity" is a directory or
/// a file.
pub fn file_type(entity: &str) -> Result<Entity, CoutilsError> {
    let mut result: Entity = Entity::Unknown;
    if Path::new(entity).exists() {
        if Path::new(entity).is_dir() {
            result = Entity::Dir;
        }
        else if Path::new(entity).is_file(){
            result = Entity::File;
        }
        else {
            result = Entity::Unknown;
        }
    }
    else {
        let e: String = format!("Entity \"{}\" does not exist.", entity);
        return Err::<Entity, CoutilsError>(CoutilsError::new(&e.to_string()));
    }
    Ok(result)
}

/// Deletes a file and returns 
/// a result type depending
/// on whether the operation succeeded.
pub fn del_file(path: &str) -> Result<(), CoutilsError> {
    let del_op = match remove_file(path) {
        Ok(del_op) => del_op,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    Ok(del_op)
}

/// Tries to copy a file from "src" to "target"
/// and returns a result type depending on whether the
/// operation succeeded or not.
pub fn file_copy(src: &str, target: &str) -> Result<(), CoutilsError> {
    let options = CopyOptions::new();
    let copy_op = match copy(src, target, &options){
        Ok(copy_op) => copy_op,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    Ok(())
}
