use actix_web::web;

use crate::handlers::app_data_handlers;

pub fn configure_app_data_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/counter", web::get().to(app_data_handlers::add_counter));
}