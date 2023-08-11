use std::net::TcpListener;
use sqlx::PgPool;
use env_logger::Env;
use zero2prod::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let config = config::get_config().expect("Failed to read configuration!");
    let conn_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port!");
    startup::run(listener, conn_pool)?.await
}