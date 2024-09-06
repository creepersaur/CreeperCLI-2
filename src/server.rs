use actix_web::{web, App, HttpServer};

use super::get::get;
use super::post::post;

pub async fn run_server(port: u16) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(get))
            .route("/", web::post().to(post))
    })
    .bind(format!("localhost:{port}"))?
    .run()
    .await
}
