/*
Coutils by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "tokio"
/// crate to use the "test"
/// macro.
#[cfg(feature="loading")]
use tokio;

/// Importing the "Path" API
/// to ascertain whether file- or
/// directory-creations were successful.
#[cfg(feature="filesystem")]
use std::path::Path;

/// Importing the "PathBuf" API
/// to ascertain whether file- or
/// directory-creations were successful.
#[cfg(feature="filesystem")]
use std::path::PathBuf;

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
/// from the "fsentity" module.
#[cfg(feature="filesystem")]
use super::fsentity::Entity;

/// Importing the "file_copy" function
/// from the "file_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::file_copy;

/// Importing the "file_is" function
/// from the "file_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::file_is;

/// Importing the "read_file" function
/// from the "file_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::read_file;

/// Importing the "download_file"
/// function to test it.
#[cfg(feature="loading")]
use super::loading::download_file;

/// Importing the "create_file" function
/// from the "file_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::create_file;

/// Importing the "write_to_file" function
/// from the "file_utils" module.
#[cfg(feature="filesystem")]
use super::file_utils::write_to_file;

/// Importing the "create_tarball" function
/// to test it.
#[cfg(feature="compression")]
use super::compression::create_tarball;

/// Importing the "create_tarball" function
/// to test it.
#[cfg(feature="compression")]
use super::compression::extract_tarball;

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

/// Importing the "folder_copy" function
/// from the "dir_utils" module.
#[cfg(feature="filesystem")]
use super::dir_utils::folder_copy;

/// Importing the "dir_is" function
/// from the "dir_utils" module.
#[cfg(feature="filesystem")]
use super::dir_utils::dir_is;

/// Importing the "del_dir" function
/// from the "dir_utils" module.
#[cfg(feature="filesystem")]
use super::dir_utils::del_dir;

/// Importing the "create_directory" function
/// from the "dir_utils" module.
#[cfg(feature="filesystem")]
use super::dir_utils::create_directory;

/// Importing the "list_dir_contents" function
/// from the "dir_utils" module.
#[cfg(feature="filesystem")]
use super::dir_utils::list_dir_contents;

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
#[cfg(feature="clone")]
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


/// Testing the "create_directory" function.
#[test]
#[cfg(feature="filesystem")]
pub fn test_create_directory() -> () {
    match create_directory("test_dir"){
        Ok(_naught) => assert_eq!(Path::new("test_dir").is_dir(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    };
}


/// Testing the "dir_move" function.
#[test]
#[cfg(feature="filesystem")]
pub fn test_dir_move() -> () {
    match create_directory("test_dir_move"){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match create_directory("subject_dir"){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match dir_move("test_dir_move", "subject_dir"){
        Ok(_naught) => assert_eq!(Path::new("subject_dir/test_dir_move").is_dir(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    }
}

/// Testing the "folder_copy" function.
#[test]
#[cfg(feature="filesystem")]
pub fn test_folder_copy() -> () {
    match create_directory("folder_copy_dir"){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match create_directory("copy_subject_dir"){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match folder_copy("folder_copy_dir", "copy_subject_dir"){
        Ok(_naught) => assert_eq!(Path::new("copy_subject_dir/folder_copy_dir").is_dir(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    }
}

/// Testing the "dir_is" function.
#[test]
#[cfg(feature="filesystem")]
pub fn test_dir_is() -> () {
    match create_directory("dir_is_test"){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    assert_eq!(dir_is("dir_is_test"), true);
}

/// Testing the "del_dir" function.
#[test]
#[cfg(feature="filesystem")]
pub fn test_del_dir() -> () {
    match create_directory("del_dir_test"){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match del_dir("del_dir_test"){
        Ok(_naught) => assert_eq!(Path::new("del_dir_test").is_dir(), false),
        Err(e) => eprintln!("{}", &e.to_string())
    }
}

/// Testing the "list_dir_contents" function.
#[test]
#[cfg(feature="filesystem")]
pub fn test_list_dir_contents() -> () {
    match list_dir_contents("src"){
        Ok(cont) => assert_eq!(cont.len(), 2),
        Err(e) => eprintln!("{}", &e.to_string())
    };
}

/// Testing the "file_move" function.
#[test]
#[cfg(feature="filesystem")]
pub fn test_file_move() -> () {
    match create_directory("file_move_test"){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match create_file("test.txt"){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match file_move("test.txt", "file_move_test"){
        Ok(_naught) => assert_eq!(Path::new("file_move_test/test.txt").is_file(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    };
}

/// Testing the "file_copy" function.
#[test]
#[cfg(feature="filesystem")]
pub fn test_file_copy() -> () {
    match create_directory("file_copy_test"){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match create_file("copy.txt"){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match file_copy("copy.txt", "file_copy_test"){
        Ok(_naught) => assert_eq!(Path::new("file_copy_test/copy.txt").is_file(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    };
}

// Testing the "create_file" function.
#[test]
#[cfg(feature="filesystem")]
pub fn test_create_file() -> () {
    match create_file("created.txt"){
        Ok(_naught) => assert_eq!(Path::new("created.txt").is_file(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    };
}

// Testing the "file_is" function.
#[test]
#[cfg(feature="filesystem")]
pub fn test_file_is() -> () {
    match create_file("exists.txt"){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    assert_eq!(file_is("exists.txt"), true);
}

// Testing the "write_to_file" function
// and the "read_file" function.
#[test]
#[cfg(feature="filesystem")]
pub fn test_file_wr() -> () {
    let contents: String = "Alyx".to_string();
    match create_file("wr_test.txt"){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match write_to_file("wr_test.txt", &contents){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match read_file("wr_test.txt"){
        Ok(cont) => assert_eq!(cont, contents),
        Err(e) => eprintln!("{}", &e.to_string())
    };
}

// Testing the "del_file" function.
#[test]
#[cfg(feature="filesystem")]
pub fn test_del_file() -> () {
    match create_file("del.txt"){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match del_file("del.txt"){
        Ok(_naught) => assert_eq!(Path::new("del.txt").is_file(), false),
        Err(e) => eprintln!("{}", &e.to_string())
    };
}

// Testing the "file_type" function.
#[test]
#[cfg(feature="filesystem")]
pub fn test_file_type() -> () {
    match create_file("type.txt"){
        Ok(_naught) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match file_type("type.txt"){
        Ok(res) => assert_eq!(res, Entity::File),
        Err(e) => eprintln!("{}", &e.to_string())
    };
}

/// Testing the "download_file"
/// function.
#[tokio::test]
#[cfg(all(feature="filesystem", feature="loading"))]
pub async fn test_download() -> (){
    let file_name: String = "file_example_MP3_700KB.mp3".to_string();
    match download_file(
        &("https://file-examples.com/wp-content/storage/2017/11/file_example_MP3_700KB.mp3".to_string()),
        &file_name
    ).await {
        Ok(_feedback) => _feedback,
        Err(e) => println!("{}", &e.to_string()) 
    };
    assert_eq!(Path::new(&file_name).exists(), true)
}

/// Testing the "create_tarball" function.
#[test]
#[cfg(all(feature="compression", feature="filesystem"))]
pub fn test_compression_creation() -> () {
    match create_directory("tarball_create_test"){
        Ok(_naught) => assert_eq!(Path::new("tarball_create_test").is_dir(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    };
    let mut pathbuf = PathBuf::new();
    pathbuf.push("tarball_create_test");
    pathbuf.push("type.text");
    match create_file(&pathbuf.display().to_string()){
        Ok(_naught) => assert_eq!(Path::new(&pathbuf.display().to_string()).is_file(), true),
        Err(e) => eprintln!("{}", &e.to_string())
    };
    match create_tarball(
        "tarball_create_test",
        "tarball_create_test.tar.gz"
    ){
        Ok(feedback) => assert_eq!(Path::new("tarball_create_test.tar.gz").exists(),true),
        Err(e) => println!("{}", e)
    };
    match del_dir("tarball_create_test"){
        Ok(feedback) => assert_eq!(Path::new("tarball_create_test").exists(),false),
        Err(e) => println!("{}", e)
    };
}

/// Testing the "extract_tarball" function.
#[test]
#[cfg(all(feature="compression", feature="filesystem"))]
pub fn test_extraction() -> () {
    match extract_tarball(
        "tarball_create_test.tar.gz",
        "tarball_create_test"
    ){
        Ok(feedback) => assert_eq!(Path::new("tarball_create_test").exists(),true),
        Err(e) => println!("{}", e)
    };
}