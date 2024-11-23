/*
Coutils by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the "modules"
/// folder as a module.
pub mod modules;

/// Re-exporting this
/// crate's time
/// module.
#[cfg(feature = "time")]
pub use modules::time::*;

/// Declaring the module for
/// cloning Git repositories
/// and exporting it.
#[cfg(feature = "clone")]
pub use modules::clone::*;

/// Re-exporting this
/// crate's error-handling
/// module.
pub use modules::error::*;

/// Re-exporting the module
/// provide network loading
/// functionality.
#[cfg(feature="loading")]
pub use modules::loading::*;

/// Re-exporting the module
/// for storing filesystem
/// entity info.
#[cfg(feature="filesystem")]
pub use modules::fsentity::*;

/// Declaring the module for
/// working with integers
/// and re-exporting it.
pub use modules::int_utils::*;

/// Declaring the module for
/// working with directories
/// and re-exporting it.
#[cfg(feature = "filesystem")]
pub use modules::dir_utils::*;

/// Declaring the module for
/// working with vectors
/// and re-exporting it.
pub use modules::vec_utils::*;

/// Declaring the module for
/// working with files
/// and re-exporting it.
#[cfg(feature = "filesystem")]
pub use modules::file_utils::*;

/// Re-exporting the module
/// to handle file-compression.
#[cfg(feature="compression")]
pub use modules::compression::*;

/// Declaring the module for
/// working with strings
/// and re-exporting it.
pub use modules::string_utils::*;
