use actix_web::{http::header, HttpResponse, Responder};
use colored::Colorize;
use serde_json::{json, Value};
use std::mem::drop;

use crate::{
    filesystem::{self, get_cwd},
    get::map_tree,
    settings::get_settings,
    ROOT,
};

pub async fn post(body: String) -> impl Responder {
    let data: Value = serde_json::from_str(&body).unwrap();
    let data = &data["data"];

    match data[0].as_str().expect("Failed to get data[0].") {
        "__INIT__" => {
            println!(
                "{} {} {} connection established!",
                data[1].to_string().split("\"").collect::<String>(),
                "|".dimmed(),
                "CreeperCLI".blue()
            );
    
            let root = match ROOT.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner()
            };
            let files = filesystem::get_root_files(root.as_str());
            drop(root);
    
            return HttpResponse::Ok()
                .append_header((header::CONTENT_TYPE, "application/json"))
                .body(map_tree(files));
        },
        "__SETTINGS__" => {
            let cwd = get_cwd();
            if let Ok(settings) = get_settings(&cwd) {
                let json = json!(settings).to_string();

                return HttpResponse::Ok().body(json);
            } else {
                println!("{}", "Failed to read settings! [Add a `creeper.toml` file into the main folder to silence.]".yellow());
            }
        },
        "__FILE__" => {
            let data = [
                data[1].to_string(),
                data[2].to_string(),
                data[3].to_string(),
            ];

            filesystem::write_file(
                data[0][1..data[0].len() - 1].to_string(),
                data[1][1..data[1].len() - 1].to_string(),
                data[2][1..data[2].len() - 1].to_string(),
            );

            return HttpResponse::Ok().body(r#"{"File added": "SUCCESS"}"#);
        },
        "__SOURCEMAP__" => {
            let data = [
                data[1].to_string(),
                data[2].to_string()
            ];
            filesystem::write_sourcemap(
                data[0][1..data[0].len() - 1].to_string(),
                data[1][1..data[1].len() - 1].to_string()
            )
        }
        _ => {}
    }

    HttpResponse::Ok().body(r#"{"Hello": "World"}"#)
}
