use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// web::Data is a form of dependacy injection (type-map mapping Any to TypeId::of)
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let req_id = Uuid::new_v4();
    log::info!(
        "[{req_id}]Adding '{}' '{}' as a new subscriber",
        form.email,
        form.name
    );
    log::trace!("[{req_id}] Saving new subscriber details into db");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => {
            log::trace!("[{req_id}] New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("[{req_id}] Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
