use actix_web::{web, HttpResponse, Responder};

use crate::{AppState, AppStateWithMutex};

pub async fn print_token(app_data: web::Data<AppState>) -> impl Responder{
    HttpResponse::Ok().body(app_data.token.clone())
}

pub async fn add_counter(app_data: web::Data<AppStateWithMutex>) -> impl Responder {
    let mut counter = app_data.counter.lock().unwrap();

    *counter += 1;

    HttpResponse::Ok().body(format!("Counter: {}", counter))
}