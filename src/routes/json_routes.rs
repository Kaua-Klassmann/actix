use actix_web::web;

use crate::handlers::json_handlers;

pub fn configure_json_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/json", web::get().to(json_handlers::json));
}