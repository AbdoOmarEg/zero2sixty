use crate::routes::{greet, health_check, subscribe};
use axum::routing::get;
use axum::routing::post;
use axum::Router;

use sqlx::PgPool;

// #[derive(Clone)]
// struct AppState {
//     db_pool: PgPool,
// }

pub async fn run(
    // listener: std::net::TcpListener,
    db_pool: PgPool,
) -> Result<Router, std::io::Error> {
    let app = Router::new()
        .route("/", get(greet))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(db_pool);

    Ok(app)
}
