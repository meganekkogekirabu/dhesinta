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
use axum::extract::{Json, Path};
use axum::http::{Response, StatusCode};
use axum_login::AuthSession;
use chrono::Utc;
use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::api::model::HttpModel;
use crate::entry::{Entry, Field};
use crate::error::Error;
use crate::{Database, Nanoid};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryPayload {
    dictionary_id: Nanoid,
    word: String,
    fields: HashMap<String, String>,
}

#[derive(Serialize)]
struct EntryResponse<'a> {
    #[serde(flatten)]
    entry: Entry,

    fields: HashMap<&'a String, &'a String>,
}

#[async_trait]
impl HttpModel for Entry {
    type Payload = Json<EntryPayload>;

    async fn http_post(
        mut session: AuthSession<crate::state::State>,
        Json(payload): Self::Payload,
    ) -> Result<Response<String>, StatusCode> {
        let id = Nanoid::default();

        let EntryPayload {
            fields,
            dictionary_id,
            word,
        } = payload;

        let fields: Vec<_> = fields.iter().map(|f| Field::from_tuple(&id, f)).collect();

        let now = Utc::now();

        let entry = Entry {
            id,
            owner_id: session.user.unwrap().id,
            dictionary_id,
            word,
            fields: Arc::new(fields),
            created_at: now,
            updated_at: now,
        };

        let entry = Arc::new(entry);

        match entry.clone().write(&mut session.backend).await {
            Ok(_) => match serde_json::to_string(&entry) {
                Ok(entry) => {
                    let mut response = Response::new(entry);
                    let status = response.status_mut();
                    *status = StatusCode::CREATED;
                    Ok(response)
                }
                Err(e) => {
                    error!("error serialising user: {e}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            },
            Err(e) => match e {
                Error::Database(dbe) => {
                    if dbe.as_database_error().unwrap().is_foreign_key_violation() {
                        Err(StatusCode::BAD_REQUEST)
                    } else {
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
        }
    }

    async fn http_get(
        Path(id): Path<String>,
        mut session: AuthSession<crate::state::State>,
    ) -> Result<Response<String>, StatusCode> {
        let entry = Entry::load(id, &mut session.backend)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let entry = entry.ok_or(StatusCode::NOT_FOUND)?;

        let fields = entry.fields.clone();
        let fields: HashMap<_, _> = fields.iter().map(|f| (&f.key, &f.value)).collect();

        let entry = EntryResponse { fields, entry };

        let entry = serde_json::to_string(&entry).map_err(|e| {
            error!("could not serialise entry: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(Response::new(entry))
    }
}
