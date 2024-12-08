use actix_web::{web::Path, HttpResponse, Responder};

pub async fn hello(name: Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}", name))
}