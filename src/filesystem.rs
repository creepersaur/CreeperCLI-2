use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use colored::Colorize;
use serde_json::json;
use toml::Table;
use fs::File;
use unescape::unescape;

use crate::{
    get::GLOBAL_DATA,
    ROOT
};

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
            let path = i.0;
            let mut content = fs::read_to_string(&path).expect("Failed to get file content.");
            let extension = path.extension().expect("Failed to get path extension!");

            if extension == "toml" {
                let parsed = content.parse::<Table>();
                if let Ok(parsed) = parsed {
                    content = json!(parsed).to_string();
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

pub fn get_root_files(root: &str) -> Vec<FileTree> {
    let cwd = get_cwd();
    let game = format!("{cwd}\\{root}");
    get_tree(game.as_str())
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

pub fn write_file(path: &mut String, contents: String, file_type: &mut String) {
    let contents = unescape(&contents).unwrap_or(contents);
    let root = match ROOT.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner()
    };

    let new_parent_path = format!(
        "{root}\\{}",
        path.replace(".", "\\"),
    );
    drop(root);

    let mut new_file_path = format!(
        "{}.{}",
        new_parent_path,
        match file_type.as_str() {
            "server" => "server.lua",
            "client" => "server.lua",
            "json" => "json",
            "toml" => "toml",
            _ => "lua"
        }
    );

    let mut dir_path = new_parent_path.split("\\").collect::<Vec<&str>>();
    dir_path.remove(dir_path.len() - 1);
    
    let dir_path = dir_path.join("\\");

    if new_file_path.ends_with("lua") && Path::new(format!("{new_file_path}u").as_str()).exists() {
        new_file_path = format!("{new_file_path}u");
    }

    if !Path::new(&dir_path).exists() {
        let mut builder = fs::DirBuilder::new();
        builder.recursive(true);
        if let Err(_) = fs::DirBuilder::new().create(&dir_path) {
            println!("{}", format!("Failed to build directory `{}`.", dir_path).red())
        }
    }

    if let Ok(mut new_file) = File::create(&new_file_path) {
        new_file.write_all(contents.as_bytes())
                .expect("Failed to write to file.")
    } else {
        println!("{}", format!("{} {}", "Failed to create file:".red(), path.purple()))
    }

    let data = match GLOBAL_DATA.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // Recover from poisoned mutex
    };

    alter_tree(&mut (*data).clone(), new_file_path, contents.clone());
    drop(data);
}

fn alter_tree(x: &mut Vec<FileTree>, new_path: String, contents: String) {
    for i in x {
        match i {
            FileTree::File(path, ref mut content) => {
                if path.to_str().expect("Failed to unwrap path.") == new_path.as_str() {
                    println!("{}", content.yellow());
                    *content = contents.clone();
                    println!("{}", content.purple());
                    return
                }
            },
            FileTree::Directory(_, ref mut tree) => {
                alter_tree(tree, new_path.clone(), contents.clone());
            }
        }
    }
}