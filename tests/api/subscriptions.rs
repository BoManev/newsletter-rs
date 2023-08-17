use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::utils::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = "name=bo%20manev&email=bo_manev%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;
    let response = app.post_subscriptions(body.into()).await;

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
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=bo%20manev", "missing the email"),
        ("email=bo_manev%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "Failed with 400 Bad Request with payload {}",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=bo_manev%40gmail.com", "empty name"),
        ("name=bo%20manev&email=", "empty email"),
        ("name=bo%20manev&email=not-an-email", "invalid email"),
    ];

    for (body, desc) in test_cases {
        let response = app.post_subscriptions(body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "Failed to return 400 Bad Request with payload {}",
            desc
        );
    }
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    let app = spawn_app().await;
    let body = "name=bo%20manev&email=bo_manev%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    let app = spawn_app().await;
    let body = "name=bo%20manev&email=bo_manev%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;

    let email_req = &app.email_server.received_requests().await.unwrap()[0];
    let body: serde_json::Value =
        serde_json::from_slice(&email_req.body).unwrap();

    let get_link = |s: &str| {
        let links: Vec<_> = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect();
        assert_eq!(links.len(), 1);
        links[0].as_str().to_owned()
    };

    let html_link = get_link(&body["HtmlBody"].as_str().unwrap());
    let text_link = get_link(&body["TextBody"].as_str().unwrap());
    assert_eq!(html_link, text_link);
}
