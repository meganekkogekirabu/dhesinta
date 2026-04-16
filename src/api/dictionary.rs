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

use async_trait::async_trait;
use axum::{Json, Router};
use axum::http::{Response, StatusCode};
use axum_login::{login_required, AuthSession};
use chrono::Utc;
use log::error;
use serde::Deserialize;
use std::sync::Arc;
use axum::routing::{delete, get, post};
use crate::api::model::HttpModel;
use crate::database::{Dictionary, DictionaryVisibility};
use crate::error::Error;
use crate::Nanoid;
use crate::database::model::DatabaseModel;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DictPayload {
    name: String,
    description: Option<String>,
    owner_id: Nanoid,
    visibility: DictionaryVisibility,
}

#[async_trait]
impl HttpModel for Dictionary {
    type Payload = Json<DictPayload>;

    async fn create(
        mut session: AuthSession<crate::state::State>,
        Json(payload): Self::Payload,
    ) -> Result<Response<String>, StatusCode> {
        let now = Utc::now();

        let dictionary = Self {
            name: payload.name,
            description: payload.description,
            visibility: payload.visibility,
            owner_id: payload.owner_id,
            created_at: now,
            updated_at: now,
            ..Default::default()
        };

        let dictionary = Arc::new(dictionary);

        match dictionary
            .clone()
            .database_write(&mut session.backend)
            .await
        {
            Ok(_) => match serde_json::to_string(&dictionary) {
                Ok(dictionary) => {
                    let mut response = Response::new(dictionary);
                    let status = response.status_mut();
                    *status = StatusCode::CREATED;
                    Ok(response)
                }
                Err(e) => {
                    error!("error serialising dictionary: {e}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            },
            Err(Error::Database(e)) => {
                if e.as_database_error().unwrap().is_foreign_key_violation() {
                    Err(StatusCode::BAD_REQUEST)
                } else {
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    fn make() -> Router<crate::state::State> {
        Router::new()
            .route("/", post(Self::create))
            .route("/", delete(Self::delete))
            .route_layer(login_required!(crate::state::State))
            .route("/", get(Self::query))
            .route("/{id}", get(Self::get))
    }
}
