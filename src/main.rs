mod http;
mod config;
mod domain;

use std::sync::{Arc, Mutex};

use crate::domain::ledger::Ledger;
use crate::{config::Config, http::routes::format_listen_addr};
use crate::http::create_router;

use tracing::{info, Level};
use tracing_subscriber::EnvFilter;
use anyhow::Result;

pub struct AppState {
    pub ledger: Arc<Mutex<Ledger>>,
}

#[tokio::main]
async fn main() -> Result<()>{
    init_tracing();

    let config = Config::from_env()?;

    info!(?config, "Loaded Configuration");

    let app = create_router();
    let address = format_listen_addr(config.http_port);
    let listener = tokio::net::TcpListener::bind(address).await?;

    info!("Listening on http://{}", address);

    axum::serve(listener, app).await?;

    Ok(())
}

fn init_tracing() {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("mini_ledger=info"));

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_env_filter(env_filter)
        .with_target(false)
        .compact()
        .init();
}