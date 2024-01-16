use askama_axum::IntoResponse;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{routing::get, Router};
use serde::Deserialize;
#[tokio::main]
async fn main() {
    println!("hello");
    // build our application with a route
    let app = Router::new()
        .route("/", get(greet))
        .route("/:name", get(greet))
        .route("/health_check", get(_health_check));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct PathParameter {
    name: Option<String>,
}

async fn greet(Path(path_parameter): Path<PathParameter>) -> impl IntoResponse {
    format!(
        "hello {:?}",
        path_parameter.name.unwrap_or("word".to_string())
    )
}

async fn _health_check() -> impl IntoResponse {
    StatusCode::OK
}
