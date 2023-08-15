use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
/// TODO: change to std::sync::Once

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "z2p-test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(
            subscriber_name,
            default_filter_level,
            std::io::stdout,
        );
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(
            subscriber_name,
            default_filter_level,
            std::io::sink,
        );
        init_subscriber(subscriber)
    }
});

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind port!");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut config =
        get_configuration().expect("Failed to read configuration.");
    config.database.database_name = Uuid::new_v4().to_string();
    let conn_pool = configure_dababase(&config.database).await;

    let server =
        run(listener, conn_pool.clone()).expect("Failed to launch backend!");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: conn_pool,
    }
}

pub async fn configure_dababase(config: &DatabaseSettings) -> PgPool {
    let mut conn = PgConnection::connect(
        &config.connection_string_no_db().expose_secret(),
    )
    .await
    .expect("Failed to connect to Postgres.");
    conn.execute(
        format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str(),
    )
    .await
    .expect("Failed to create database.");

    let conn_pool =
        PgPool::connect(&config.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&conn_pool)
        .await
        .expect("Failed to migrate the database.");
    conn_pool
}

#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to send request!");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // setup
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=bo%20manev&email=bo_manev%40gmail.com";
    // exercise
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request!");

    // verify (& teardown)
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "bo_manev@gmail.com");
    assert_eq!(saved.name, "bo manev");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // table-driven testing (parametrised test)
    // setup
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=bo%20manev", "missing the email"),
        ("email=bo_manev%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    // exercise
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send request!");
        // verify (&teardown)
        assert_eq!(
            400,
            response.status().as_u16(),
            "Failed with 400 Bad Request, payload {}",
            error_message
        );
    }
}
