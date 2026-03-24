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
use axum_login::AuthManagerLayerBuilder;
use axum_login::tower_sessions::{Expiry, SessionManagerLayer};
use dhesinta::config::NetConfig;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use tower_sessions::cookie::{Key, time::Duration};
use tower_sessions_sqlx_store::SqliteStore;

use crate::api;
use crate::state::State;

pub struct App {
    state: State,
}

impl App {
    pub fn new(state: State) -> Self {
        Self { state }
    }

    pub async fn serve(self) -> dhesinta::Result<()> {
        let session_store = SqliteStore::new(self.state.db.clone());
        session_store.migrate().await?;

        let key = Key::generate();

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::days(1)))
            .with_signed(key);

        let auth_layer = AuthManagerLayerBuilder::new(self.state.clone(), session_layer).build();

        let app = Router::new()
            .with_state(self.state.clone())
            .nest("/api", api::make().with_state(self.state.clone()))
            .layer(auth_layer);

        let NetConfig { hostname, port } = self.state.config.net;
        let hostname = Ipv4Addr::from_str(&hostname)?;
        let hostname = IpAddr::V4(hostname);
        let addr = SocketAddr::new(hostname, port);

        let listener = tokio::net::TcpListener::bind(addr).await?;

        axum::serve(listener, app).await?;

        Ok(())
    }
}
