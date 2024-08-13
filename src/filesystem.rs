use std::env;
use std::fs;
use std::path::PathBuf;
use colored::Colorize;
use serde_json::json;
use toml::Table;

#[derive(Debug, Clone, PartialEq)]
pub enum FileTree {
    File(PathBuf, String),             // (path, content)
    Directory(PathBuf, Vec<FileTree>), // (path, files)
}

pub fn get_tree(root: &str) -> Vec<FileTree> {
    let mut tree: Vec<FileTree> = vec![];

    for i in get_files(root).unwrap() {
        if i.1 {
            tree.push(FileTree::Directory(i.0.clone(), get_tree(&i.0.to_string_lossy())));
        } else {
            let mut path = i.0;
            let mut content = fs::read_to_string(&path).expect("Failed to get file content.");
            let extension = path.extension().expect("Failed to get path extension!");

            if extension == "toml" {
                let parsed = content.parse::<Table>();
                if let Ok(parsed) = parsed {
                    content = json!(parsed).to_string();
                    path.set_extension("toml");
                } else {
                    println!("{}{}", "Unable to parse `toml` file: ".red(), path.to_str().unwrap().blue());
                    continue;
                }
            }

            tree.push(FileTree::File(path.clone(), content));
        }
    }

    tree
}


pub fn get_files(path: &str) -> Result<Vec<(PathBuf, bool)>, ()> {
    let children = fs::read_dir(path)
        .map_err(|_| ())?  // Failed to get files, return Error
        .into_iter()
        .scan((), |_, x| {  // Scan does create an iterator that skips if closure returns None
            if let Ok(x) = x { // If it is a valid DirEntry (Not an error)
                let path = x.path(); // Get path
                let is_dir = x.path().is_dir();  // Get boolean
                return Some((path, is_dir))  // Add entry to the scan iter
            }
            None  // If it is a DirEntry Error, yeet the file from the list
        })
        .collect();  // Collect into the vector
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