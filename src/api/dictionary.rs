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

use crate::api::model::HttpModel;
use crate::dictionary::{Dictionary, DictionaryVisibility};
use crate::error::Error;
use crate::{Database, Nanoid};
use async_trait::async_trait;
use axum::Json;
use axum::http::StatusCode;
use axum_login::AuthSession;
use chrono::Utc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

    async fn http_post(
        mut session: AuthSession<crate::state::State>,
        Json(payload): Self::Payload,
    ) -> StatusCode {
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

        match dictionary.write(&mut session.backend).await {
            Err(Error::Database(e)) => {
                if e.as_database_error().unwrap().is_foreign_key_violation() {
                    StatusCode::BAD_REQUEST
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Ok(_) => StatusCode::CREATED,
        }
    }
}
