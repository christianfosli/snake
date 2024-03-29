use std::{env, error::Error, net::SocketAddr};

use axum::{
    routing::{get, post},
    Router,
};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use init_tracing_opentelemetry::tracing_subscriber_ext;
use mongodb::{options::ClientOptions, Client, Database};
use tokio::net::TcpListener;

mod health;
mod submit;
mod top_ten;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber_ext::init_subscribers()?;

    let db = get_db_handle().await?;

    let app = Router::new()
        .route("/topten", get(top_ten::handle_top_ten))
        .route("/submit", post(submit::submit))
        .route("/readyz", get(health::ready))
        .route("/livez", get(health::live))
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .with_state(db);

    let addr = env::var("LISTEN_ADDR")
        .unwrap_or(String::from("[::]:3000"))
        .parse::<SocketAddr>()?;

    tracing::info!(?addr, "Service started successfully");

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

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
