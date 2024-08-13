use actix_web::{http::header, HttpResponse, Responder};
use colored::Colorize;
use serde_json::Value;

use crate::{filesystem, get::map_tree};

pub async fn post(body: String) -> impl Responder {
    let data: Value = serde_json::from_str(&body).unwrap();
    let data = &data["data"];

    if data[0] == "__INIT__" {
        println!("{} {} {} connection established!", data[1].to_string().split("\"").collect::<String>(), "|".dimmed(), "CreeperCLI".blue());

        return HttpResponse::Ok()
            .append_header((header::CONTENT_TYPE, "application/json"))
            .body(
                map_tree(filesystem::get_game_files())
            )
    } else if data[0] == "__DELETED__" {
        println!("{} was just deleted!!!", data[1].to_string().red());
    }
    HttpResponse::Ok().body("{Hello}")
}