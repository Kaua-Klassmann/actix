use actix_web::web;

use crate::handlers::static_handlers;

pub fn configure_static_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(static_handlers::hello));
}