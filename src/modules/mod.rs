/*
Coutils by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting the "time"
/// module.
#[cfg(feature="time")]
pub mod time;

/// Exporting the "tests"
/// module.
#[cfg(test)]
pub mod tests;

/// Exporting the module
/// for working with Git
/// repositories.
#[cfg(feature="clone")]
pub mod clone;

/// Exporting this
/// crate's error-handling
/// module.
pub mod error;

/// Exporting the module
/// providing network loading
/// functionality.
#[cfg(feature="loading")]
pub mod loading;

/// Exporting the module
/// for storing filesystem
/// entity info.
#[cfg(feature="filesystem")]
pub mod fsentity;

/// Exporting the module
/// for working with 
/// integers.
pub mod int_utils;

/// Exporting the module
/// for working with 
/// directories.
#[cfg(feature="filesystem")]
pub mod dir_utils;

/// Exporting the module
/// for working with 
/// vectors.
pub mod vec_utils;

/// Exporting the module
/// for working with 
/// files.
#[cfg(feature="filesystem")]
pub mod file_utils;

/// Exporting the module
/// to handle file compression.
#[cfg(feature="compression")]
pub mod compression;

/// Exporting the module
/// for working with 
/// strings.
pub mod string_utils;
