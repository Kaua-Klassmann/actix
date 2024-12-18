use actix_web::{App, HttpServer};
use actix_cors::Cors;

mod handlers;
mod routes;

use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();

    let app_port: u16 = dotenv::var("APP_PORT")
        .ok()
        .and_then(|p: String| p.parse().ok())
        .unwrap_or(3000);

    println!("Server running in port {}", app_port);

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .configure(static_routes::configure_static_routes)
            .configure(path_routes::configure_path_routes)
            .configure(query_routes::configure_query_routes)
            .configure(json_routes::configure_json_routes)
    })
    .bind(("127.0.0.1", app_port))
    .unwrap()
    .run()
    .await
}
