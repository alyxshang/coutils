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

/// Importing the "PathBuf"
/// structure to save results
/// from walking a directory.
use std::path::PathBuf;

/// Importing the method
/// to copy files
/// from the "fs-extra"
/// crate.
use fs_extra::file::copy;

/// Importing Rust's "remove_file"
/// function from the "fs" module
/// to remove files.
use std::fs::remove_file;

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

/// Importing this crate's error
/// structure.
use super::error::CoutilsError;

/// We need this structure to
/// perform copying operations
/// on files.
use fs_extra::file::CopyOptions;

/// Tries to move a file from "src" to "target"
/// and returns a "Result" type depending on whether the
/// operation succeeded or not.
pub fn file_move(src: &str, target: &str) -> Result<(), CoutilsError> {
    let options = CopyOptions::new();
    let mut new_buf: PathBuf = PathBuf::new();
    new_buf.push(target);
    let _move_op: u64 = match move_file(src, &new_buf.display().to_string(), &options){
        Ok(_move_op) => _move_op,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    Ok(())
}

/// Checks whether a file exists and
/// returns a boolean to that effect.
pub fn file_is(filename: &str) -> bool {
    Path::new(filename).exists()
}

/// Tries to create a file and returns
/// a "Result" type depending on whether the
/// operation succeeded or not.
pub fn create_file(filename: &str) -> Result<(), CoutilsError>{
    let _new_file: File = match File::create(filename){
        Ok(_new_file) => _new_file,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    Ok(())
}

/// Tries to write a string to a file and returns
/// a "Result" type depending on whether the
/// operation succeeded or not.
pub fn write_to_file(
    filename: &str, 
    contents: &str
) -> Result<(), CoutilsError> {
    if file_is(filename) == true {
        let write_op: () = match write(filename, contents) {
            Ok(write_op) => write_op,
            Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
        };
        Ok(write_op)
    }
    else {
        let e: String = format!("The file \"{}\" could not be found.", filename);
        Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    }
}

/// Tries to read a file and return
/// the contents as a string. A "Result"
/// type is returned.
pub fn read_file(file_name: &str) -> Result<String, CoutilsError> {
    if file_is(file_name) == true {
        let result: String = match read_to_string(file_name) {
            Ok(result) => result,
            Err(e) => {
                return Err::<String, CoutilsError>(CoutilsError::new(&e.to_string()));
            }
        };
        Ok(result)
    }
    else {
        let e: String = format!("The file \"{}\" could not be found.", file_name);
        Err::<String, CoutilsError>(CoutilsError::new(&e.to_string()))
    }
}

/// Checks whether "entity" is a directory or
/// a file. A "Result" type is returned depending on whether
/// the file or directory exists or not.
pub fn file_type(entity: &str) -> Result<Entity, CoutilsError> {
    if Path::new(entity).exists() {
        if Path::new(entity).is_dir() {
            Ok(Entity::Dir)
        }
        else if Path::new(entity).is_file(){
            Ok(Entity::File)
        }
        else {
            Ok(Entity::Unknown)
        }
    }
    else {
        let e: String = format!("Entity \"{}\" does not exist.", entity);
        Err::<Entity, CoutilsError>(CoutilsError::new(&e.to_string()))
    }
}

/// Deletes a file and returns 
/// a "Result" type depending
/// on whether the operation succeeded or not.
pub fn del_file(path: &str) -> Result<(), CoutilsError> {
    let _del_op: () = match remove_file(path) {
        Ok(_del_op) => _del_op,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    Ok(())
}

/// Tries to copy a file from "src" to "target"
/// and returns a "Result" type depending on whether the
/// operation succeeded or not.
pub fn file_copy(src: &str, target: &str) -> Result<(), CoutilsError> {
    let options = CopyOptions::new();
    let mut new_buf: PathBuf = PathBuf::new();
    new_buf.push(target);
    let _copy_op: u64 = match copy(src, new_buf.display().to_string(), &options){
        Ok(_copy_op) => _copy_op,
        Err(e) => return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()))
    };
    Ok(())
}