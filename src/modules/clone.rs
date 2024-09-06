/*
Coutils by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Repository"
/// structure from the "git2"
/// crate to work with Git repositories.
use git2::Repository;

/// Importing the "dir_is"
/// method from the directory
/// module to check if a
/// directory exists.
use super::dir_utils::dir_is;

/// Importing Coutils's error
/// struct.
use super::error::CoutilsError;

/// Importing the "create_directory"
/// method from the "dir_utils"
/// module to create a directory.
use super::dir_utils::create_directory;

/// Clones a Git repository from the given host,
/// owner, repository, and branch. Returns an error
/// if this operation fails.
pub fn clone_repo(
    dir: &str, 
    host: &str, 
    owner: &str, 
    repo: &str, 
    branch: &str
) -> Result<(), CoutilsError> {
    if dir_is(&dir.to_string()){
        let err_msg: &String = &format!("The directory \"{}\" already exists!", dir);
        return Err::<(), CoutilsError>(CoutilsError::new(&err_msg.to_string()));
    }
    else {
        match create_directory(&dir.to_string()){
            Ok(_x) => {},
            Err(e) => {return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()));}
        }
        let repo_url: &String = &format!("https://{}.com/{}/{}.git#{}", host, owner, repo, branch);
        let _repo = match Repository::clone(repo_url, dir) {
            Ok(_repo) => _repo,
            Err(e) => {return Err::<(), CoutilsError>(CoutilsError::new(&e.to_string()));}
        };
        Ok(())
    }
}
