use std::sync::Mutex;

use actix_web::{web::Data, App, HttpServer};
use actix_cors::Cors;

mod handlers;
mod routes;

struct AppStateWithMutex {
    counter: Mutex<i32>
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Server running in port 3000");

    let counter: Data<AppStateWithMutex> = Data::new(AppStateWithMutex {
        counter: Mutex::new(0)
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .app_data(counter.clone())
            .configure(routes::static_routes::configure_static_routes)
            .configure(routes::path_routes::configure_path_routes)
            .configure(routes::app_data_routes::configure_app_data_routes)
    })
    .bind(("127.0.0.1", 3000))
    .unwrap()
    .run()
    .await
}
