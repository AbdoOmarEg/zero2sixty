use axum::extract::Path;
use axum::response::IntoResponse;
use serde::Deserialize;

// greet deserializer
#[derive(Deserialize)]
pub struct PathParameter {
    name: Option<String>,
}

// greet endpoint
pub async fn greet(Path(path_parameter): Path<PathParameter>) -> impl IntoResponse {
    format!(
        "hola {:?}",
        path_parameter.name.unwrap_or("word".to_string())
    )
}
