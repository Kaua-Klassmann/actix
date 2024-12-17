use actix_web::web;

use crate::handlers::path_handlers;

pub fn configure_path_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/path/{name}", web::get().to(path_handlers::hello));
}