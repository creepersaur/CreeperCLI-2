use std::env;
use std::fmt::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum FileTree {
    File(String, String),
    Directory(String, Vec<FileTree>),
}

pub fn get_tree(root: &str) -> Vec<FileTree> {
    let mut tree: Vec<FileTree> = vec![];

    for i in get_files(root).unwrap() {
        if i.1 {
            tree.push(FileTree::Directory(i.0.clone(), get_tree(&i.0)));
        } else {
            tree.push(FileTree::File(
                i.0.clone(),
                fs::read_to_string(i.0).expect("Failed to get file content."),
            ));
        }
    }

    tree
}

pub fn get_files(path: &str) -> Result<Vec<(String, bool)>, Error> {
    let paths = fs::read_dir(path).expect("Failed to get files.");
    let mut children: Vec<(String, bool)> = vec![];

    for path in paths {
        let path = path.expect("DirEntry error.").path();

        let mut is_dir = false;
        if let Ok(metadata) = fs::metadata(path.clone()) {
            is_dir = metadata.is_dir();
        }

        children.push((path.to_str().unwrap().to_string(), is_dir));
    }

    Ok(children)
}

pub fn get_cwd() -> String {
    let cwd = env::current_dir()
        .expect("Failed to get current working directory.")
        .to_str()
        .expect("Failed to convert cwd to str.")
        .to_owned();

    cwd
}

pub fn extract_file_info(path: &str) -> (String, String) {
    let path = Path::new(path);
    let file_name = path.file_name().and_then(|s| s.to_str()).map(String::from);
    let extension = path.extension().and_then(|s| s.to_str()).map(String::from);
    (file_name.unwrap(), extension.unwrap())
}
