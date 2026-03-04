mod app;
mod config;
mod error;
mod health;
mod routes;

use crate::config::Config;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = Config::from_env().map_err(|e| anyhow::anyhow!(e))?;
    let pool = db::connect(&cfg.database_url).await?;

    let app = app::build_app(pool);

    let addr = format!("{}:{}", cfg.host, cfg.port);
    tracing::info!("listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
