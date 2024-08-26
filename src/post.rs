use actix_web::{http::header, HttpResponse, Responder};
use colored::Colorize;
use serde_json::{json, Value};

use crate::{
    filesystem::{self, get_cwd},
    get::map_tree,
    settings::get_settings,
};

pub async fn post(body: String) -> impl Responder {
    let data: Value = serde_json::from_str(&body).unwrap();
    let data = &data["data"];

    if data[0] == "__INIT__" {
        println!(
            "{} {} {} connection established!",
            data[1].to_string().split("\"").collect::<String>(),
            "|".dimmed(),
            "CreeperCLI".blue()
        );

        return HttpResponse::Ok()
            .append_header((header::CONTENT_TYPE, "application/json"))
            .body(map_tree(filesystem::get_root_files("game")));
    } else if data[0] == "__SETTINGS__" {
        let cwd = get_cwd();
        if let Ok(settings) = get_settings(&cwd) {
            let json = json!(settings).to_string();

            return HttpResponse::Ok().body(json);
        } else {
            println!("{}", "Failed to read settings!".red());
        }
    } else if data[0] == "__FILE__" {
        let data = [
            data[1].to_string(),
            data[2].to_string(),
            data[3].to_string(),
        ];

        // println!("ARGH {}", format!("the filetype is: {}", data[2]).cyan());
        // println!("FILE WRITE REQUESTED: {}", data[1].purple());

        filesystem::write_file(
            &mut data[0][1..data[0].len() - 1].to_string(),
            data[1][1..data[1].len() - 1].to_string(),
            &mut data[2][1..data[2].len() - 1].to_string(),
        );

        return HttpResponse::Ok().body(r#"{"File added": "SUCCESS"}"#);
    }

    HttpResponse::Ok().body(r#"{"Hello": "World"}"#)
}
