use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber(
        "zero2prod".into(),
        "info".into(),
        std::io::stdout,
    );
    telemetry::init_subscriber(subscriber);

    let config = configuration::get_configuration()
        .expect("Failed to read configuration!");
    let conn_pool = PgPool::connect_lazy(
        config.database.connection_string().expose_secret(),
    )
    .expect("Failed to connect to Postgres.");
    let address =
        format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind port!");
    startup::run(listener, conn_pool)?.await
}
