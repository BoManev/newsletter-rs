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

    let response = reqwest::get(confirmation_links.plain_text).await.unwrap();

    assert_eq!(response.status().as_u16(), 200);
}
