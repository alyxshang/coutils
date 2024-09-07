/*
Coutils by Alyx Shang.
Licensed under the FSL v1.
*/

/// Using Rust's standard
/// "Path" API from the
/// "path" module.
use std::path::Path;

/// Importing the method
/// to copy directories
/// from the "fs-extra"
/// crate.
use fs_extra::dir::copy;

/// Importing the "create_dir"
/// function from Rust's
/// "fs" module.
use std::fs::create_dir;

/// Importing the "read_dir"
/// function to list the contents 
/// of a directory.
use std::fs::read_dir;

/// Importing the enum
/// that describes types
/// of filesystem entities.
use super::fsentity::Entity;

/// Importing Rust's "remove_dir_all"
/// function from the "fs" module
/// to remove directories.
use std::fs::remove_dir_all;

/// We import the "move_dir"
/// method from the "fs_extra"
/// crate.
use fs_extra::dir::move_dir;

/// Importing this crate's error
/// structure.
use super::error::CoutilsError;

/// Importing the enum
/// to store information
/// about filesystem entities.
use super::fsentity::FileEntry;

/// We need this entity to
/// perform copying operations
/// on directories.
use fs_extra::dir::CopyOptions;

/// Tries to copy a folder from "src" to "target"
/// and returns a `Result` type depending on whether the
/// operation succeeded or not.
pub fn folder_copy(src: &str, target: &str) -> Result<(), CoutilsError> {
    let options = CopyOptions::new();
    let _copy_op = match copy(src, target, &options) {
        Ok(_copy_op) => _copy_op,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    Ok(())
}

/// Attempts to move a directory from "src" to "target".
/// A `Result` type is returned depending on whether the operation
/// suceeded or not.
pub fn dir_move(src: &str, target: &str) ->  Result<(), CoutilsError> {
    let options = CopyOptions::new();
    let _move_op = match move_dir(src, target, &options) {
        Ok(_move_op) => _move_op,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    Ok(())
}

/// Tries to create a new directory and returns
/// a `Result` type depending on whether the
/// operation succeeded or not.
pub fn create_directory(path: &str) ->  Result<(), CoutilsError> {
    let _new_dir = match create_dir(path) {
        Ok(_new_dir) => _new_dir,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    Ok(())
}

/// Checks whether a directory exists.
/// Returns a boolean to this effect.
pub fn dir_is(dir: &str) -> bool {
    return Path::new(dir).is_dir();
}

/// A method to return the contents of a directory.
/// Returns this information in the form of a vector of the
/// "FileEntry" entity. Skips all invalid or non-existent entries.
pub fn list_dir_contents(dir: &str) -> Result<Vec<FileEntry>, CoutilsError> {
    let mut result: Vec<FileEntry> = Vec::new();
    let dirs = match read_dir(dir) {
        Ok(dirs) => dirs,
        Err(e) => {
            return Err::<Vec<FileEntry>, CoutilsError>(CoutilsError::new(&e.to_string()));
        }
    };
    for item in dirs {
        match item {
            Ok(dir_item) => {
                let path_item: &String = &dir_item.path().display().to_string();
                if Path::new(path_item).is_dir() {
                    result.push(
                        FileEntry::new(
                            path_item,
                            &Entity::Dir
                        )
                    );
                }
                else {
                    result.push(
                        FileEntry::new(
                            &path_item,
                            &Entity::File
                        )
                    );
                }
            },
            Err(e) => {
                return Err::<Vec<FileEntry>, CoutilsError>(CoutilsError::new(&e.to_string()));
            }
        };
    }
    return Ok(result);
}

/// Deletes a directory and returns 
/// a `Result` type depending on whether 
/// the operation succeeded or not.
pub fn del_dir(path: &str) -> Result<(), CoutilsError> {
    let _del_op = match remove_dir_all(path){
        Ok(_del_op) => _del_op,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    Ok(())
}