use actix_web::{web, HttpResponse, Responder};

pub async fn hello(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}", name))
}