use axum::extract::State;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Form};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// for making it in a beautiful pretty span
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(pool, form_data),
    fields(
        subscriber_email = %form_data.email,
        subscriber_name= %form_data.name
    )
)]
pub async fn subscribe(
    State(pool): State<PgPool>,
    Form(form_data): Form<FormData>,
) -> impl IntoResponse {
    match insert_subscriber(&pool, &form_data).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(pool, form_data)
)]
pub async fn insert_subscriber(pool: &PgPool, form_data: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at) 
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form_data.email,
        form_data.name,
        // Utc::now(),
        chrono::Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
