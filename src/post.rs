use actix_web::{HttpResponse, Responder};

pub async fn post(body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("Received POST data: {}", body))
}