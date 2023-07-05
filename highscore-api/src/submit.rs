use axum::{extract::State, http::StatusCode, Json};
use highscore_types::{HighScoreDocument, HighScoreDto};
use mongodb::Database;

const GENERIC_DB_ERROR: &str = "An error occured trying to persist highscore to database";

pub async fn submit(
    State(db): State<Database>,
    Json(payload): Json<HighScoreDto>,
) -> Result<(StatusCode, Json<HighScoreDto>), (StatusCode, String)> {
    let collection = db.collection::<HighScoreDocument>("highscore");

    let doc = HighScoreDocument::try_from_dto(&payload).map_err(|e| {
        tracing::warn!(?e, "Mapping from highscore dto to doc failed validation");
        (StatusCode::UNPROCESSABLE_ENTITY, e)
    })?;

    collection.insert_one(doc, None).await.map_err(|e| {
        tracing::error!(?e, "Failed to persist highscore to database");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            GENERIC_DB_ERROR.to_string(),
        )
    })?;

    Ok((StatusCode::CREATED, Json(payload)))
}
