use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::email_client::EmailClient;
use crate::routes;

pub fn run(
    listener: TcpListener,
    conn: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let email_client = web::Data::new(email_client); // uses Arc to provide a thread-safe shared reference to an email_client
    let db_pool = web::Data::new(conn);
    let server = HttpServer::new(move || {
        // new app instance per worker thread
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
