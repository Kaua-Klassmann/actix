use actix_web::{web::Data, HttpResponse, Responder};

use crate::AppStateWithMutex;

pub async fn add_counter(app_data: Data<AppStateWithMutex>) -> impl Responder {
    let mut counter = app_data.counter.lock().unwrap();

    *counter += 1;

    HttpResponse::Ok().body(format!("Counter: {}", counter))
}