use std::{collections::HashSet, env, error::Error};

use bson::{doc, oid::ObjectId, DateTime};
use futures::TryStreamExt;
use highscore_types::HighScoreDocument;
use init_tracing_opentelemetry::tracing_subscriber_ext;
use mongodb::{
    options::{ClientOptions, FindOptions},
    Client, Database,
};
use time::OffsetDateTime;

const TO_KEEP_COUNT: u8 = 15;

async fn do_cleanup(db: &Database) -> Result<(), Box<dyn Error>> {
    let collection = db.collection::<HighScoreDocument>("highscore");

    let find_opts = FindOptions::builder()
        .sort(doc! { "score": -1})
        .skip(Some(u64::from(TO_KEEP_COUNT)))
        .build();

    let ok_to_delete_all_time = collection
        .find(None, find_opts.clone())
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let ok_to_delete_all_time = ok_to_delete_all_time
        .iter()
        .filter_map(|hs| hs.id)
        .collect::<HashSet<_>>();

    let start_of_this_year = DateTime::builder()
        .year(OffsetDateTime::now_utc().year())
        .month(1)
        .day(1)
        .build()?;

    let ok_to_delete_this_year = collection
        .find(doc! { "timestamp": {"$gte": start_of_this_year}}, find_opts)
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    let ok_to_delete_this_year = ok_to_delete_this_year
        .iter()
        .filter_map(|hs| hs.id)
        .collect::<HashSet<_>>();

    let to_delete = ok_to_delete_this_year
        .intersection(&ok_to_delete_all_time)
        .copied()
        .collect::<Vec<_>>();

    if !to_delete.is_empty() {
        tracing::info!(?to_delete, "Ready to delete highscores");

        let res = collection
            .delete_many(doc! { "_id": {"$in": to_delete}}, None)
            .await?;

        tracing::info!(
            deleted_count = res.deleted_count,
            "Successfully deleted no-longer-needed highscores"
        );
    } else {
        tracing::info!("Nothing to delete");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber_ext::init_subscribers()?;

    let db = get_db_handle().await?;

    tracing::info!("Cleanup job started successfully");

    do_cleanup(&db).await?;

    tracing::info!("Cleanup job completed successfully");

    Ok(())
}

async fn get_db_handle() -> Result<Database, mongodb::error::Error> {
    let connstr = env::var("DB_CONNSTR").unwrap_or_else(|e| {
        tracing::warn!(?e, "DB_CONNSTR not set. Defaulting to localhost db.");
        String::from("mongodb://root:secret@localhost:27017/")
    });

    let mongo_client = Client::with_options(ClientOptions::parse(connstr).await?)?;
    Ok(mongo_client.database("snake-highscore"))
}
