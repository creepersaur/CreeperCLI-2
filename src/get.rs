use actix_web::{http::header, HttpResponse, Responder};
use lazy_static::lazy_static;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Mutex;

use crate::{
    filesystem::{self, FileTree},
    ROOT
};

lazy_static! {
    pub static ref GLOBAL_DATA: Mutex<Vec<FileTree>> = Mutex::new(vec![]);
}

pub async fn get() -> impl Responder {
    let root = match ROOT.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner()
    };
    let mut tree = filesystem::get_root_files(&root);
    
    drop(root);

    let mut data = match GLOBAL_DATA.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // Recover from poisoned mutex
    };
    let mut old_tree = data.clone();

    *data = tree.clone();
    // drop(data);

    tree = trim_tree(&mut tree, &mut old_tree);

    HttpResponse::Ok()
        .append_header((header::CONTENT_TYPE, "application/json"))
        .body(map_tree(tree))
}

pub fn map_tree(tree: Vec<FileTree>) -> String {
    let mut file_structure: HashMap<String, String> = HashMap::new();

    for i in tree {
        if let FileTree::File(path, content) = i {
            let (mut name, extension) = (
                path.file_name().expect("Failed to get file_name").to_str().unwrap().to_string(),
                path.extension().expect("Failed to get extension!")
            );

            if extension == "toml" || extension == "json" {
                name = format!("{name}.json");
            }
            file_structure.insert(name, content);
        } else if let FileTree::Directory(path, files) = i {
            let new_path = path.to_string_lossy();
            let root = match ROOT.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner()
            };
            let game_index = new_path.find(root.as_str()).unwrap();
            let (_, end) = new_path.split_at(game_index);
            
            drop(root);

            file_structure.insert(end.to_string(), map_tree(files));
        }
    }

    let json_structure: Value = json!(file_structure);
    serde_json::to_string_pretty(&json_structure).unwrap()
}

fn trim_tree(tree: &mut Vec<FileTree>, old_tree: &mut Vec<FileTree>) -> Vec<FileTree> {
    let mut new_tree: Vec<FileTree> = vec![];
    let file_types = vec!["luau", "lua", "json", "toml"];

    for x in 0..tree.len() {
        if old_tree.contains(&tree[x]) {
            match tree[x].clone() {
                FileTree::File(path, c1) => {
                    if let FileTree::File(_, c2) = old_tree[x].clone() {
                        if !file_types.contains(&path.extension().unwrap().to_str().unwrap()) {
                            continue;
                        }
                        if c1 == c2 {
                            continue;
                        }
                    }
                }
                FileTree::Directory(path, mut f1) => {
                    if x >= old_tree.len() {
                        continue
                    }
                    if let FileTree::Directory(_, mut f2) = old_tree[x].clone() {
                        let trimmed = trim_tree(&mut f1, &mut f2);
                        if trimmed.len() < 1 {
                            continue;
                        }
                        old_tree[x] = FileTree::Directory(path, trimmed);
                    }
                }
            }
        }

        new_tree.push(tree[x].clone());
    }

    new_tree
}
