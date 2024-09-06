/*
Coutils by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Path" API
/// to ascertain whether file- or
/// directory-creations were successful.
use std::path::Path;

/// Importing the "get_time"
/// function from the "time"
/// module.
#[cfg(feature="clone")]
use super::clone::clone_repo;

/// Importing the "is_int"
/// function from the "int_utils"
/// module.
use super::int_utils::is_int;

/// Importing the "has_item"
/// function from the "vec_utils"
/// module.
use super::vec_utils::has_item;

/// Importing the "raise_to"
/// function from the "int_utils"
/// module.
use super::int_utils::raise_to;

/// Importing the "parse_int"
/// function from the "int_utils"
/// module.
use super::int_utils::parse_int;

/// Importing the "get_index"
/// function from the "vec_utils"
/// module.
use super::vec_utils::get_index;

/// Importing the "has_index"
/// function from the "vec_utils"
/// module.
use super::vec_utils::has_index;

/// Importing the "file_move" function
/// from the "file_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::file_move;

/// Importing the "Entity" enum
/// from the "file_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::Entity;

/// Importing the "file_copy" function
/// from the "file_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::file_copy;

/// Importing the "file_is" function
/// from the "file_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::file_is;

/// Importing the "create_file" function
/// from the "file_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::create_file;

/// Importing the "write_to_file" function
/// from the "file_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::write_to_file;

/// Importing the "read_file" function
/// from the "file_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::read_file;

/// Importing the "file_type" function
/// from the "file_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::file_type;

/// Importing the "del_file" function
/// from the "file_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::del_file;

/// Importing the "dir_move" function
/// from the "dir_utils" module.
#[cfg(feature="filesystem")]
use super::dir_utils::dir_move;

/// Importing the "FileEntry" enum
/// from the "dir_utils" module.
#[cfg(feature="filesystem")]
use super::dir_utils::FileEntry;

/// Importing the "folder_copy" function
/// from the "dir_utils" module.
#[cfg(feature="filesystem")]
use super::dir_utils::folder_copy;

/// Importing the "dir_is" function
/// from the "dir_utils" module.
#[cfg(feature="filesystem")]
use super::dir_utils::dir_is;

/// Importing the "create_directory" function
/// from the "dir_utils" module.
#[cfg(feature="filesystem")]
use super::dir_utils::create_directory;

/// Importing the "list_dir_contents" function
/// from the "dir_utils" module.
#[cfg(feature="filesystem")]
use super::dir_utils::list_dir_contents;

/// Importing the "del_dir" function
/// from the "dir_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::del_dir;

/// Importing the "reverse_vec"
/// function from the "vec_utils"
/// module.
use super::vec_utils::reverse_vec;

/// Importing the "remove_last"
/// function from the "vec_utils"
/// module.
use super::vec_utils::remove_last;

/// Importing the "get_last_item"
/// function from the "vec_utils"
/// module.
use super::vec_utils::get_last_item;

/// Importing the "clean_split"
/// function from the "string_utils"
/// module.
use super::string_utils::clean_split;

/// Testing the "clean_split" function.
#[test]
pub fn test_clean_split() -> () {
    let test_string: String = String::from("Hello World!");
    let split_char: String = String::from(" ");
    let result_vec: Vec<String> = vec![
        String::from("Hello"),
        String::from("World!")
    ];
    assert_eq!(
        clean_split(&test_string, &split_char),
        result_vec
    );
}

/// Testing the "get_index" function.
#[test]
pub fn test_get_index() -> () {
    let index: usize = 1;
    let test_vec: Vec<usize> = vec![1,2,3,4];
    match get_index(&test_vec, &2){
        Ok(res) => {
            assert_eq!(
                res,
                index
            );
        },
        Err(e) => {
            println!("{}", e);
        }
    };
}

/// Testing the "has_index" function.
#[test]
pub fn test_has_index() -> () {
    let result: bool = true;
    let test_vec: Vec<usize> = vec![1,2,3,4];
    assert_eq!(
        has_index(&test_vec,&1),
        result
    );
}

/// Testing the "remove_last" function.
#[test]
pub fn test_remove_last() -> () {
    let mut test_vec_full: Vec<usize> = vec![1,2,3,4];
    let test_vec_stripped: Vec<usize> = vec![1,2,3];
    assert_eq!(
        remove_last(&mut test_vec_full),
        test_vec_stripped
    );
}

/// Testing the "is_int" function.
#[test]
pub fn test_is_int() -> () {
    let true_string: String = String::from("1");
    let false_string: String = String::from("A");
    assert_eq!(
        is_int(&true_string),
        true
    );
    assert_eq!(
        is_int(&false_string),
        false
    );
}

/// Testing the "parse_int" function.
#[test]
pub fn test_parse_int() -> () {
    let true_string: String = String::from("1");
    let false_string: String = String::from("A");
    assert_eq!(
        parse_int(&true_string),
        1
    );
    assert_eq!(
        parse_int(&false_string),
        0
    );
}

/// Testing the "reverse_vec" function.
#[test]
pub fn test_reverse_vec() -> () {
    let test_vec_norm: Vec<usize> = vec![1,2,3,4];
    let test_vec_rev: Vec<usize> = vec![4,3,2,1];
    assert_eq!(
        reverse_vec(
            &test_vec_norm
        ),
        test_vec_rev
    );
}

/// Testing the "raise_to" function.
#[test]
pub fn test_raise_to() -> () {
    let base: u32 = 2;
    let power: u32 = 3;
    let result: u32 = 8;
    assert_eq!(
        raise_to(
            &base,
            &power
        ),
        result
    );
}

/// Testing the "get_last_item" function.
#[test]
pub fn test_get_last_item() -> () {
    let test_vec: Vec<usize> = vec![1,2,3,4];
    let last: usize = 4;
    assert_eq!(
        get_last_item(
            &test_vec
        ),
        last
    );
}

/// Testing the "has_item" function.
#[test]
pub fn test_has_item() -> () {
    let test_vec: Vec<usize> = vec![1,2,3,4,5];
    let existing_test_item: usize = 2;
    let non_existing_test_item: usize = 6;
    assert_eq!(
        has_item(&test_vec, &existing_test_item),
        true
    );
    assert_eq!(
        has_item(&test_vec, &non_existing_test_item),
        false
    );
}

/// Testing the "clone_repo" function.
#[test]
pub fn test_clone_repo() -> () {
    match clone_repo(
        "coutils",
        "github.com",
        "alyxshang",
        "coutils",
        "main"
    ) {
        Ok(_repo) => assert_eq!(Path::new("coutils").is_dir(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    };
}

/// Testing the "file_move" function.
#[test]
pub fn test_file_move() -> () {
    let src: &str = ".gitignore";
    let dest: &str = "src/.gitignore";
    match file_move(
        src, dest
    ){
        Ok(_move) => assert_eq!(Path::new(dest).is_file(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    };
}

/// Testing the "file_is" function.
#[test]
pub fn test_file_is() -> () {
    assert_eq!(file_is("README.markdown"), true);
}

/// Testing the "create_file" function.
#[test]
pub fn test_create_file() -> () {
    let new_file: &str = ".hello";
    match create_file(new_file){
        Ok(_op) => assert_eq!(Path::new(new_file).is_file(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    }
}

/// Testing the "file_type" function.
#[test]
pub fn test_file_type() -> () {
    let some_file: &str = "README.markdown";
    assert_eq!(file_type(some_file), Entity::File);
}

/// Testing the "del_file" function.
#[test]
pub fn test_del_file() -> () {
    let new_file: &str = ".hello";
    match del_file(new_file){
        Ok(_op) => assert_eq!(Path::new(new_file).is_file(), false),
        Err(e) => eprintln!("{}", &e.to_string())
    }
}

/// Testing the "file_copy" function.
#[test]
pub fn test_file_copy() -> () {
    let old_file: &str = ".hello";
    let new_copy: &str = "src/.hello";
    match file_copy(old_file, new_copy){
        Ok(_op) => assert_eq!(Path::new(new_copy).is_file(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    }
}

/// Testing the "write_to_file" function
/// and the "read_file" function.
#[test]
pub fn test_file_wr() -> () {
    let my_file: &str = ".hello";
    let contents: &str = "Lorem ipsum.\n";
    match write_to_file(my_file, contents){
        Ok(_op) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match read_file(my_file){
        Ok(cont) => assert_eq!(cont, contents),
        Err(e) => eprintln!("{}", &e.to_string())
    };
}

/// Testing the "create_directory" function.
#[test]
pub fn test_create_directory() -> () {
    match create_directory(".test"){
        Ok(_op) => assert_eq!(Path::new(".test").is_dir(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    }
}

/// Testing the "folder_copy" function.
#[test]
pub fn test_folder_copy() -> () {
    match folder_copy(".test", "src/.test"){
        Ok(_op) => assert_eq!(Path::new("src/.test").is_dir(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    }
}

/// Testing the "del_dir" function.
#[test]
pub fn test_del_dir() -> () {
    match del_dir("src/.test"){
        Ok(_op) => assert_eq!(Path::new("src/.test").is_dir(), false),
        Err(e) => eprintln!("{}", &e.to_string())
    }
}

/// Testing the "dir_is" function.
#[test]
pub fn test_dir_is() -> () {
    assert_eq!(Path::new("src").is_dir(), false);
}

/// Testing the "list_dir_contents" function.
#[test]
pub fn test_list_dir_contents() -> () {
    let mut res: Vec<FileEntry> = Vec::new();
    res.push(FileEntry::new("modules", &Entity::Dir));
    res.push(FileEntry::new("lib.rs", &Entity::File));
    match list_dir_contents("src"){
        Ok(contents) => assert_eq!(res, contents),
        Err(e) => eprintln!("{}", &e.to_string())
    }
}

/// Testing the "dir_move" function.
#[test]
pub fn test_dir_move() -> () {
    match dir_move(".github", "src/.github"){
        Ok(contents) => assert_eq!(Path::new("src/.github").is_dir(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    }
}