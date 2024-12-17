use actix_web::web;

use crate::handlers::query_handlers;

pub fn configure_query_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/query", web::get().to(query_handlers::hello));
}