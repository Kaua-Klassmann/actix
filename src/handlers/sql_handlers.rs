use actix_web::{web::Data, HttpResponse, Responder};
use sea_orm::{ConnectionTrait, Statement};

use crate::config::app_state::AppState;

pub async fn sql(app_data: Data<AppState>) -> impl Responder {
    let _res = app_data.db
        .query_all(
            Statement::from_string(app_data.db.get_database_backend(), "SELECT * FROM user")
        ).await.unwrap();

    dbg!(_res);

    HttpResponse::Ok().body("OK")
}
