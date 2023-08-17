use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::{email_client::EmailClient, *};

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
    let conn_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.database.with_db());

    let address =
        format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind port!");

    let sender_email =
        config.email_client.sender().expect("Invalid sender email");
    let email_client =
        EmailClient::new(config.email_client.base_url, sender_email);

    startup::run(listener, conn_pool, email_client)?.await?;
    Ok(())
}
