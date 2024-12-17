use actix_web::{web::Query, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    name: String
}

pub async fn hello(query: Query<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}", query.name))
}