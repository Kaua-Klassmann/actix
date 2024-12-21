use actix_web::{web::Data, HttpResponse, Responder};
use sea_orm::{ConnectionTrait, DatabaseBackend::MySql, Statement};

use crate::config::app_state::AppState;

pub async fn sql(app_data: Data<AppState>) -> impl Responder {
    let _res = app_data.db
        .query_all(
            Statement::from_string(MySql, "SELECT * FROM user")
        ).await.unwrap();

    dbg!(_res);

    HttpResponse::Ok().body("OK")
}