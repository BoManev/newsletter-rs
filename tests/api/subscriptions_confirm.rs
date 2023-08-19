use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::utils::spawn_app;

#[tokio::test]
async fn configurations_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;

    let response =
        reqwest::get(&format!("{}/subscriptions/confirm", app.address))
            .await
            .unwrap();

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
    let app = spawn_app().await;
    let body = "name=bo%20manev&email=bo_manev%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;

    let email_req = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_req);
    println!("HERE!!!{}", confirmation_links.plain_text.clone());
    let response = reqwest::get(confirmation_links.plain_text).await.unwrap();

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn request_to_confirmation_link_confirms_a_subscriber() {
    let app = spawn_app().await;
    let body = "name=bo%20manev&email=bo_manev%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;
    let email_req = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_req);

    reqwest::get(confirmation_links.plain_text)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "bo_manev@gmail.com");
    assert_eq!(saved.name, "bo manev");
    assert_eq!(saved.status, "confirmed");
}

// What happens if a user tries to subscribe twice? Make sure that they receive two confirmation emails;
// What happens if a user clicks on a confirmation link twice?
// What happens if the subscription token is well-formatted but non-existent?
// Add validation on the incoming token, we are currently passing the raw user input straight into a query (thanks sqlx for protecting us from SQL injections <3);
// Use a proper templating solution for our emails (e.g. tera);
