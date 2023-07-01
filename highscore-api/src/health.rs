use axum::{http::StatusCode, response::IntoResponse};

pub async fn ready() -> impl IntoResponse {
    (StatusCode::OK, "Ready")
}

pub async fn live() -> impl IntoResponse {
    (StatusCode::OK, "Alive")
}
