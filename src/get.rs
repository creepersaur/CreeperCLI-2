use crate::filesystem::{self, FileTree};
use actix_web::{http::header, HttpResponse, Responder};
use serde_json::{json, Value};
use std::collections::HashMap;

use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref GLOBAL_DATA: Mutex<Vec<FileTree>> = Mutex::new(vec![]);
}

pub async fn get() -> impl Responder {
    let cwd = filesystem::get_cwd();
    let game = format!("{cwd}\\game");
    let mut tree = filesystem::get_tree(game.as_str());
    
    let mut data = match GLOBAL_DATA.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), // Recover from poisoned mutex
    };
    let mut old_tree = data.clone();
    *data = tree.clone();
    let tree = trim_tree(&mut tree, &mut old_tree);

    HttpResponse::Ok()
        .append_header((header::CONTENT_TYPE, "application/json"))
        .body(map_tree(tree))
}

fn map_tree(tree: Vec<FileTree>) -> String {
    let file_types = vec!["luau", "lua", "json", "toml"];
    let mut file_structure = HashMap::new();

    for i in tree {
        if let FileTree::File(path, content) = i {
            let (name, extension) = filesystem::split_file_path(&path);

            if file_types.contains(&extension.as_str()) {
                file_structure.insert(format!("{name}"), content);
            }
        } else if let FileTree::Directory(path, files) = i {
            let new_path : &mut str = &mut path.to_string();
            let game_index = path.find("game").unwrap();
            let (_, end) = new_path.split_at(game_index);

            file_structure.insert(end.to_string(), map_tree(files));
        }
    }

    let json_structure: Value = json!(file_structure);
    serde_json::to_string_pretty(&json_structure).unwrap()
}

fn trim_tree(tree: &mut Vec<FileTree>, old_tree: &mut Vec<FileTree>) -> Vec<FileTree> {
    let mut new_tree: Vec<FileTree> = vec![];

    for x in 0..tree.len() {
        if old_tree.contains(&tree[x]) {
            match tree[x].clone() {
                FileTree::File(_, c1) => if let FileTree::File(_, c2) = old_tree[x].clone() {
                    if c1 == c2 {
                        continue;
                    }
                },
                FileTree::Directory(path, mut f1) => {
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