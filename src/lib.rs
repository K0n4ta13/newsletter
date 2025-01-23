mod routes;
mod configuration;

use std::env;
use std::fs::File;
use std::sync::Mutex;
use anyhow::Context;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter
};
pub use routes::serve;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

pub async fn run() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    // let log_file = File::create("zero2prod.log")?;

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_env("RUST_LOG")
                .unwrap_or(format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer()
            // .with_writer(Mutex::new(log_file))
        )
        .init();

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&env::var("DATABASE_URL")?)
        .await
        .context("failed to connect to database")?;

    let state = AppState { db };

    let app = serve()
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}