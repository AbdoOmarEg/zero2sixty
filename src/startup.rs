use crate::routes::{greet, health_check, subscribe};
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use tower_http::trace::TraceLayer;
use tracing;

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
        .layer(
            // thanks to https://github.com/tokio-rs/axum/discussions/2273
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let request_id = uuid::Uuid::new_v4();
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str)
                    .unwrap_or("no matched_path");
                tracing::span!(
                    tracing::Level::INFO,
                    "request",
                    method = tracing::field::display(request.method()),
                    uri = tracing::field::display(request.uri()),
                    version = tracing::field::debug(request.version()),
                    request_id = tracing::field::display(request_id),
                    matched_path = tracing::field::display(matched_path)
                )
            }),
        )
        .with_state(db_pool);

    Ok(app)
}
