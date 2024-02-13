use axum::extract::State;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Form};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// #[allow(unused_variables)]
pub async fn subscribe(
    State(pool): State<PgPool>,
    Form(form_data): Form<FormData>,
) -> impl IntoResponse {
    println!("happened here hello there");
    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at) 
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form_data.email,
        form_data.name,
        Utc::now(),
    )
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
    // StatusCode::OK.into_response()
}
