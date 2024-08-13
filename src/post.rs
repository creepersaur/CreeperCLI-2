use actix_web::{http::header, HttpResponse, Responder};
use colored::Colorize;
use serde_json::Value;

use crate::{filesystem, get::map_tree};

pub async fn post(body: String) -> impl Responder {
    let data: Value = serde_json::from_str(&body).unwrap();
    let data = &data["data"];

    if data[0] == "__INIT__" {
        println!("{} {} {} connection established!", data[1].to_string().split("\"").collect::<String>(), "|".dimmed(), "CreeperCLI".blue());

        let cwd = filesystem::get_cwd();
        let game = format!("{cwd}\\game");
        let tree = filesystem::get_tree(game.as_str());

        return HttpResponse::Ok()
            .append_header((header::CONTENT_TYPE, "application/json"))
            .body(map_tree(tree))
    }
    HttpResponse::Ok().body("Hello")
}