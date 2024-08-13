use std::env;
use std::fmt::Error;
use std::fs;
use colored::Colorize;
use serde_json::json;
use toml::Table;

#[derive(Debug, Clone, PartialEq)]
pub enum FileTree {
    File(String, String),             // (path, content)
    Directory(String, Vec<FileTree>), // (path, files)
}

pub fn get_tree(root: &str) -> Vec<FileTree> {
    let mut tree: Vec<FileTree> = vec![];

    for i in get_files(root).unwrap() {
        if i.1 {
            tree.push(FileTree::Directory(i.0.clone(), get_tree(&i.0)));
        } else {
            let mut path = i.0;
            let mut content = fs::read_to_string(&path).expect("Failed to get file content.");
            let (_, extension) = split_file_path(&path);

            if extension == "toml" {
                let parsed = content.parse::<Table>();
                if let Ok(parsed) = parsed {
                    content = json!(parsed).to_string();
                    path.replace_range(path.len() - 4.., "json");
                } else {
                    println!("{}{}", "Unable to parse `toml` file: ".red(), path.blue());
                    continue;
                }
            }

            tree.push(FileTree::File(path.clone(), content));
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

pub fn split_file_path(path: &str) -> (String, String) {
    let mut split: Vec<&str> = path.split(".").collect();

    let extension = split
        .last()
        .expect("Failed to get extension?")
        .to_string();

    split.remove(split.len() - 1);
    let file_name = split.join(".");

    (file_name, extension)
}


    // let path = Path::new(path);
    // let file_name = path.file_name().and_then(|s| s.to_str()).map(String::from);
    // let extension = path.extension().and_then(|s| s.to_str()).map(String::from);
    // (file_name.unwrap(), extension.unwrap())