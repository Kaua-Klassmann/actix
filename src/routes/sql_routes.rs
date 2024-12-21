use actix_web::web;

use crate::handlers::sql_handlers;

pub fn configure_sql_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/sql", web::get().to(sql_handlers::sql));
}