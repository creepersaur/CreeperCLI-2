use colored::Colorize;
use fs::File;
use serde_json::json;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use toml::Table;
use unescape::unescape;

use crate::CWD;
use crate::{get::GLOBAL_DATA, ROOT};

#[derive(Debug, Clone, PartialEq)]
pub enum FileTree {
    File(PathBuf, String),             // (path, content)
    Directory(PathBuf, Vec<FileTree>), // (path, files)
}

pub fn get_tree(root: &str) -> Vec<FileTree> {
    let mut tree: Vec<FileTree> = vec![];

    for i in get_files(root).unwrap() {
        if i.1 {
            tree.push(FileTree::Directory(
                i.0.clone(),
                get_tree(&i.0.to_string_lossy()),
            ));
        } else {
            let path = i.0;
            let mut content = fs::read_to_string(&path).expect("Failed to get file content.");
            let extension = match path.extension() {
                Some(ext) => ext,
                _ => continue
            };

            if extension == "toml" {
                let parsed = content.parse::<Table>();
                if let Ok(parsed) = parsed {
                    content = json!(parsed).to_string();
                } else {
                    println!(
                        "{}{}",
                        "Unable to parse `toml` file: ".red(),
                        path.to_str().unwrap().blue()
                    );
                    continue;
                }
            }

            tree.push(FileTree::File(path.clone(), content));
        }
    }

    tree
}

pub fn get_root_files(root: &str) -> Vec<FileTree> {
    let game = CWD.join(root);
    get_tree(game.to_str().expect("Failed to get root."))
}

pub fn get_files(path: &str) -> Result<Vec<(PathBuf, bool)>, ()> {
    let children = fs::read_dir(path)
        .map_err(|_| ())? // Failed to get files, return Error
        .into_iter()
        .scan((), |_, x| {
            // Scan does create an iterator that skips if closure returns None
            if let Ok(x) = x {
                // If it is a valid DirEntry (Not an error)
                let path = x.path(); // Get path
                let is_dir = x.path().is_dir(); // Get boolean
                return Some((path, is_dir)); // Add entry to the scan iter
            }
            None // If it is a DirEntry Error, yeet the file from the list
        })
        .collect(); // Collect into the vector
    Ok(children)
}

pub fn get_cwd() -> PathBuf {
    env::current_dir().expect("Failed to get current working directory.")
}

pub fn write_file(path: String, contents: String, file_type: String) {
    let contents = unescape(&contents).unwrap_or(contents);
    let root = match ROOT.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let root_path = (*root).clone();
    drop(root);

    let new_parent_path = format!(
        "{root_path}/{path}.{}",
        match file_type.as_str() {
            "server" => "server.lua",
            "client" => "client.lua",
            "json" => "json",
            "toml" => "toml",
            _ => "lua",
        }
    ).get_path();

    let mut new_file_path = new_parent_path.clone();
    let mut dir_path = new_parent_path;
    dir_path.pop();

    if !Path::new(&dir_path).exists() {
        println!("{} {} Building directory: `{}`.", "[DIR_BUILDER]".blue(), "|".dimmed(), dir_path.display().to_string().purple());
        build_dir(dir_path);
    }

    let extension = new_file_path.extension().unwrap().to_str().unwrap().to_string();

    if extension == "lua"
        && Path::new(format!("{}u", new_file_path.display()).as_str()).exists()
    {
        new_file_path.set_extension(format!(
            "{}u",
            new_file_path.extension().unwrap().to_str().unwrap()
        ));
    }

    if !Path::new(&new_file_path).exists() && extension == "lua" {
        new_file_path.set_extension(format!(
            "{}u",
            new_file_path.extension().unwrap().to_str().unwrap()
        ));
    }

    create_file(&new_file_path, &contents);

    let data = match GLOBAL_DATA.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // Recover from poisoned mutex
    };

    alter_tree(
        &mut (*data).clone(),
        new_file_path.to_str().unwrap().to_string(),
        contents.clone(),
    );
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
                    return;
                }
            }
            FileTree::Directory(_, ref mut tree) => {
                alter_tree(tree, new_path.clone(), contents.clone());
            }
        }
    }
}

pub fn write_sourcemap(data: String) {
    let sourcemap = format!("{}{data}{}", "{", "}");

    create_file(
        &"sourcemap.json",
        &sourcemap
    );
}

pub fn write_project(game_name: String) {
    let project = format!(
        r#"{{
    "name": "{game_name}",
    "tree": {{
        "$className": "DataModel"
    }}
}}"#
    );

    create_file(
        &"default.project.json",
        &project
    )
}

pub trait StrPath: AsRef<OsStr> {
    fn get_path(&self) -> PathBuf {
        PathBuf::from(&self)
    }

    fn get_bytes(&self) -> &[u8] {
        return &[];
    }
}

impl StrPath for String {
    fn get_bytes(&self) -> &[u8] {
        &self.as_bytes()
    }
}

impl StrPath for &str {
    fn get_bytes(&self) -> &[u8] {
        &self.as_bytes()
    }
}

impl StrPath for PathBuf {}

pub fn create_file(path: &impl StrPath, content: &impl StrPath) {
    let file_path = path.get_path();
    let file_name = file_path
        .file_name()
        .expect("Failed to get file_name: create_file().")
        .to_str()
        .unwrap();

    if let Err(out) = File::create(&file_path)
        .expect(format!("Failed to create `{}`.", file_name).as_str())
        .write_all(content.get_bytes())
    {
        println!(
            "{} Failed to write to: {}.",
            "[ERROR]".red(),
            format!("`{}`", file_name).purple()
        );

        println!("{}", out.to_string().dimmed());
    }
}

pub fn build_dir(path: impl StrPath) {
    let build_path = path.get_path();
    let mut builder = fs::DirBuilder::new();
    builder.recursive(true);

    if let Err(out) = builder.create(&build_path) {
        println!(
            "{} Failed to build directory: {}.",
            "[ERROR]".red(),
            format!("`{}`", build_path.display()).purple()
        );

        println!("{}", out.to_string().dimmed());
    }
}