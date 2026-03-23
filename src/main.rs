use axum::Router;
use condict::state;
use log::debug;
use std::sync::Arc;

mod api;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let conf = config::CondictConfig::load()?;

    debug!("opening pool at {}", conf.db_url);
    let state = state::State::new(conf.db_url)?;
    let state = Arc::new(state);

    let app = Router::new()
        .nest("/api", api::make(state.clone()))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
