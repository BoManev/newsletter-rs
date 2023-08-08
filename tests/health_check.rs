use sqlx::{Connection, PgConnection};
use zero2prod::*;

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind port!");
    let port = listener.local_addr().unwrap().port();
    let server = startup::run(listener).expect("Failed to launch backend!");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to send request!");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // setup
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let config = config::get_config().expect("Failed to read config!");
    let conn_str = config.database.connection_string();
    let mut conn = PgConnection::connect(&conn_str)
        .await
        .expect("Failed to connect to Postgres.");
    let db_client = reqwest::Client::new();

    let body = "name=bo%20manev&email=bo_manev%40gmail.com";

    // exercise
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request!");

    // verify (& teardown)
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut conn)
        .await
        .expect("Failed to fetch saved subscription.");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // table-driven testing (parametrised test)
    // setup
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=bo%20manev", "missing the email"),
        ("email=bo_manev%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    // exercise
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
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
