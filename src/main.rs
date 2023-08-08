use std::net::TcpListener;

use zero2prod::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = config::get_config().expect("Failed to read configuration!");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port!");
    startup::run(listener)?.await
}
