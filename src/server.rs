use actix_web::{web, App, HttpServer};

use super::get::get;
use super::post::post;

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(get))
            .route("/", web::post().to(post))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
