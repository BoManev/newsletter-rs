use actix_web::{
    http::header::{HeaderMap, HeaderValue},
    web, HttpRequest, HttpResponse, ResponseError,
};
use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use reqwest::{header, StatusCode};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::{domain::SubscriberEmail, email_client::EmailClient};

use super::error_chain_fmt;

#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
}

struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

#[tracing::instrument(
    name = "Publish a newsletter issue",
    skip(body, pool, email_client, request)
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn publish_newsletter(
    body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    request: HttpRequest,
) -> Result<HttpResponse, PublishError> {
    let credentials = basic_auth(request.headers()).map_err(PublishError::AuthError)?;
    tracing::Span::current()
        .record("username", &tracing::field::display(&credentials.username));
    let user_id = validate_credentials(credentials, &pool).await?;
    tracing::Span::current().record("user_id", &tracing::field::display(&user_id));

    let subscribers = get_confirmed_subscribers(&pool).await?;
    for sub in subscribers {
        match sub {
            Ok(sub) => {
                email_client
                    .send_email(
                        &sub.email,
                        &body.title,
                        &body.content.html,
                        &body.content.text,
                    )
                    .await
                    .with_context(|| {
                        format!("Failed to send newsletter issue to {}", &sub.email)
                    })?;
            }
            Err(error) => {
                tracing::warn!(
                    error.cause_chain = ?error,
                    "Skipping a confirmed subscriber. \
                    Their stored contact details are invalid."
                )
            }
        }
    }
    Ok(HttpResponse::Ok().finish())
}

async fn get_confirmed_subscribers(
    pool: &PgPool,
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    let confirmed_subscribers = sqlx::query!(
        r#"SELECT email FROM subscriptions 
        WHERE status = 'confirmed'"#,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| match SubscriberEmail::parse(r.email) {
        Ok(email) => Ok(ConfirmedSubscriber { email }),
        Err(error) => Err(anyhow::anyhow!(error)),
    })
    .collect();
    Ok(confirmed_subscribers)
}

async fn get_stored_credentials(
    username: &str,
    pool: &PgPool,
) -> Result<Option<(uuid::Uuid, Secret<String>)>, anyhow::Error> {
    let row = sqlx::query!(
        r#"SELECT user_id, password_hash FROM users
        WHERE username = $1"#,
        username
    )
    .fetch_optional(pool)
    .await
    .context("Failed to query for credentials")?
    .map(|row| (row.user_id, Secret::new(row.password_hash)));
    Ok(row)
}

#[derive(thiserror::Error)]
pub enum PublishError {
    #[error("Authorization failed")]
    AuthError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PublishError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            PublishError::UnexpectedError(_) => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            }
            PublishError::AuthError(_) => {
                let mut response = HttpResponse::new(StatusCode::UNAUTHORIZED);
                let header_val =
                    HeaderValue::from_str(r#"Basic realm="publish""#).unwrap();
                response
                    .headers_mut()
                    .insert(header::WWW_AUTHENTICATE, header_val);
                response
            }
        }
    }
}

struct Credentials {
    username: String,
    password: Secret<String>,
}

//rfc7617
fn basic_auth(headers: &HeaderMap) -> Result<Credentials, anyhow::Error> {
    let base64_encoded = headers
        .get("Authorization")
        .context("The 'Authorization' header was missing")?
        .to_str()
        .context("The 'Authorization' header was not a valid UTF8 string")?
        .strip_prefix("Basic ")
        .context("The authorization schema was not 'Baisc'")?;

    let decoded = base64::decode_config(base64_encoded, base64::STANDARD)
        .context("Failed to base64-decode 'Basic' credentials")?;
    let decoded = String::from_utf8(decoded)
        .context("The decode credential string is not valid UTF8")?;
    let mut credentials = decoded.splitn(2, ':');

    let username = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A username must be provided in 'Basic' auth"))?
        .to_string();
    let password = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A password must be provided in 'Basic' auth"))?
        .to_string();

    Ok(Credentials {
        username,
        password: Secret::new(password),
    })
}

#[tracing::instrument(
    name = "Validate credentials"
    skip(credentials, pool)
)]
async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<uuid::Uuid, PublishError> {
    let (user_id, expected_hash) = get_stored_credentials(&credentials.username, pool)
        .await
        .map_err(PublishError::UnexpectedError)?
        .ok_or_else(|| PublishError::AuthError(anyhow::anyhow!("Unknown username")))?;

    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || {
        current_span
            .in_scope(|| verify_password_hash(expected_hash, credentials.password))
    })
    .await
    .context("Failed to spawn blocking task")
    .map_err(PublishError::UnexpectedError)??;
    Ok(user_id)
}

#[tracing::instrument(
    name = "Verify password hash",
    skip(expected_hash, password_candidate)
)]
fn verify_password_hash(
    expected_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), PublishError> {
    let expected_hash = PasswordHash::new(expected_hash.expose_secret())
        .context("Failed to parse hash in PHC string format")
        .map_err(PublishError::UnexpectedError)?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_hash,
        )
        .context("Invalid password")
        .map_err(PublishError::AuthError)
}
