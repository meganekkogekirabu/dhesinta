/* Copyright (C) 2026  Madeleine Choi
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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
