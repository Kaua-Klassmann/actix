use actix_web::{App, HttpServer};
use actix_cors::Cors;

mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Server running in port 3000");

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .configure(routes::static_routes::configure_static_routes)
            .configure(routes::path_routes::configure_path_routes)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
