use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use migration::MigratorTrait;
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;

mod config;
mod handlers;
mod routes;

use config::app_state::AppState;
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let db: Arc<DatabaseConnection> = Arc::new(Database::connect((*config::database::OPT_DB).clone())
        .await.expect("Failed to connect in database"));
    migration::Migrator::up(&*db, None).await.unwrap();

    let app_port: u16 = dotenv::var("APP_PORT")
        .ok()
        .and_then(|p: String| p.parse().ok())
        .unwrap_or(3000);

    println!("Server running in port {}", app_port);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .app_data(web::Data::new(
                AppState {
                    db: db.clone()
                }
            ))
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
