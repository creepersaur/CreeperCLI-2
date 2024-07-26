use crate::filesystem::{self, FileTree};
use actix_web::{http::header, HttpResponse, Responder};
use serde_json::{json, Value};
use std::collections::HashMap;

pub async fn get() -> impl Responder {
    let cwd = filesystem::get_cwd();
    let game = format!("{cwd}\\game");
    let tree = filesystem::get_tree(game.as_str());

    HttpResponse::Ok()
        .append_header((header::CONTENT_TYPE, "application/json"))
        .body(map_tree(tree))
}

fn map_tree(tree: Vec<FileTree>) -> String {
    let file_types = vec!["luau", "lua", "json"];
    let mut file_structure = HashMap::new();

    for i in tree {
        if let FileTree::File(path, content) = i {
            let (name, extension) = filesystem::extract_file_info(&path);

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
