use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use bson::doc;
use futures::stream::TryStreamExt;
use mongodb::{bson::DateTime, options::FindOptions, Database};
use serde::Deserialize;
use time::OffsetDateTime;

use crate::models::{HighScoreDocument, HighScoreDto};

const GENERIC_DB_ERROR: &str = "An error occured trying to fetch highscores from the database";

#[derive(Deserialize)]
pub struct Params {
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub since: Option<OffsetDateTime>,
}

pub async fn handle_top_ten(
    Query(params): Query<Params>,
    State(db): State<Database>,
) -> Result<Json<Vec<HighScoreDto>>, (StatusCode, &'static str)> {
    let filter = params
        .since
        .map(DateTime::from_time_0_3)
        .map(|since| doc! { "timestamp": {"$gte": since}});

    let find_opts = FindOptions::builder()
        .sort(doc! { "score": -1 })
        .limit(10)
        .build();

    let collection = db.collection::<HighScoreDocument>("highscore");

    let scores = collection
        .find(filter, find_opts)
        .await
        .map_err(|e| {
            tracing::error!(?e, "Failed querying highscores to db cursor");
            (StatusCode::INTERNAL_SERVER_ERROR, GENERIC_DB_ERROR)
        })?
        .try_collect::<Vec<HighScoreDocument>>()
        .await
        .map_err(|e| {
            tracing::error!(?e, "Failed collecting highscores from db cursor");
            (StatusCode::INTERNAL_SERVER_ERROR, GENERIC_DB_ERROR)
        })?
        .iter()
        .map(|doc| HighScoreDto {
            user_name: doc.user_name.to_string(),
            score: doc.score,
        })
        .collect();

    Ok(Json(scores))
}
