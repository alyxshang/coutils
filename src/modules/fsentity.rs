/*
Coutils by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "PartialEq"
/// trait from Rust's "cmp"
/// module.
use std::cmp::PartialEq;

/// An enum to list
/// file types.
#[derive(PartialEq, Clone, Debug)]
pub enum Entity{
    Dir,
    File,
    Unknown
}

/// A data structure to represent
/// a file entry in a file system.
#[derive(PartialEq, Clone, Debug)]
pub struct FileEntry {
    pub name: String,
    pub file_type: Entity
}

/// Implementing methods
/// for the "FileEntry"
/// entity.
impl FileEntry {

    /// Convenience method
    /// to create a new instance
    /// of the "FileEntry" entity.
    pub fn new(name: &str, file_type: &Entity) -> FileEntry {
        return FileEntry { 
            name: name.to_owned(), 
            file_type: file_type.to_owned() 
        }
    }

    pub fn to_string(&self) -> String {
        let file_type: String = match &self.file_type{
            Entity::File => "File".to_string(),
            Entity::Dir => "Directory".to_string(),
            Entity::Unknown => "Unkown".to_string()
        };
        format("Type: {}, Path: {}", file_type, &self.name)
    }
}
