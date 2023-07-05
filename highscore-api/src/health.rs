use axum::{extract::State, http::StatusCode, response::IntoResponse};
use bson::doc;
use mongodb::Database;

pub async fn ready(State(db): State<Database>) -> impl IntoResponse {
    match db.run_command(doc! { "ping": 1}, None).await {
        Ok(_) => (StatusCode::OK, "Ready"),
        Err(e) => {
            tracing::error!(?e, "Readiness check failed due to database ping failed");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Degraded - Database unreachable",
            )
        }
    }
}

pub async fn live() -> impl IntoResponse {
    (StatusCode::OK, "Alive")
}
