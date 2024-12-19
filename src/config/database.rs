use lazy_static::lazy_static;
use sea_orm::ConnectOptions;
use dotenv::var;

lazy_static! {
    pub static ref OPT_DB: ConnectOptions = set_opt_db();
}

fn set_opt_db() -> ConnectOptions {
    dotenv::dotenv().ok();

    let url = var("DATABASE_URL").expect("DATABASE_URL not found");
    let max_connections = var("DATABASE_MAX_CONNECTIONS").expect("DATABASE_MAX_CONNECTIONS not found");
    let min_connections = var("DATABASE_MIN_CONNECTIONS").expect("DATABASE_MIN_CONNECTIONS not found");
    let schema = var("DATABASE_SCHEMA").expect("DATABASE_SCHEMA not found");

    let mut opt_db = ConnectOptions::new(url);
    opt_db.max_connections(max_connections.parse().unwrap());
    opt_db.min_connections(min_connections.parse().unwrap());
    opt_db.set_schema_search_path(schema);

    opt_db
}