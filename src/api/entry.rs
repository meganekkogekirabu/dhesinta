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

use crate::entry::{Entry, Field};
use crate::error::Error;
use crate::{Database, Nanoid};
use axum::extract::{Json, Path, State};
use axum::http::{Response, StatusCode};
use axum_login::AuthSession;
use chrono::Utc;
use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEntryPayload {
    dictionary_id: Nanoid,
    word: String,
    fields: HashMap<String, String>,
}

pub async fn create(
    session: AuthSession<crate::state::State>,
    Json(payload): Json<CreateEntryPayload>,
) -> StatusCode {
    let id = Nanoid::default();

    let CreateEntryPayload {
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
        fields,
        created_at: now,
        updated_at: now,
    };

    match entry.write(&session.backend.db).await {
        Ok(()) => StatusCode::CREATED,
        Err(e) => match e {
            Error::Database(dbe) => {
                if dbe.as_database_error().unwrap().is_foreign_key_violation() {
                    StatusCode::BAD_REQUEST
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        },
    }
}

#[derive(Serialize)]
struct GetEntryResponse<'a> {
    #[serde(flatten)]
    entry: Entry,

    fields: HashMap<&'a String, &'a String>,
}

pub async fn get(
    Path(id): Path<String>,
    State(state): State<crate::state::State>,
) -> Result<Response<String>, StatusCode> {
    let entry = Entry::load(id, &state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let entry = entry.ok_or(StatusCode::NOT_FOUND)?;

    let fields = entry.fields.clone();
    let fields: HashMap<_, _> = fields.iter().map(|f| (&f.key, &f.value)).collect();

    let entry = GetEntryResponse { fields, entry };

    let entry = serde_json::to_string(&entry).map_err(|e| {
        error!("could not serialise entry: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Response::new(entry))
}

pub async fn delete(
    Path(id): Path<String>,
    session: AuthSession<crate::state::State>,
) -> StatusCode {
    match Entry::load(id.to_owned(), &session.backend.db).await {
        Ok(Some(entry)) => {
            if entry.owner_id != session.user.unwrap().id {
                return StatusCode::UNAUTHORIZED;
            }
        }
        Ok(None) => return StatusCode::NOT_FOUND,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    }

    if let Err(_) = Entry::delete(id, &session.backend.db).await {
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::NO_CONTENT
    }
}
