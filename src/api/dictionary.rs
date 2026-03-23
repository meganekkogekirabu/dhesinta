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

use crate::dictionary::{Dictionary, DictionaryVisibility};
use crate::{Database, Nanoid};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use chrono::Utc;
use log::error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDictPayload {
    name: String,
    description: Option<String>,
    owner_id: Nanoid,
    visibility: DictionaryVisibility,
}

pub async fn create(
    State(state): State<crate::state::State>,
    Json(payload): Json<CreateDictPayload>,
) -> crate::Result<StatusCode> {
    let now = Utc::now();

    let dictionary = Dictionary {
        name: payload.name,
        description: payload.description,
        visibility: payload.visibility,
        owner_id: payload.owner_id,
        created_at: now,
        updated_at: now,
        ..Default::default()
    };

    dictionary
        .write(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

pub async fn get(
    Path(id): Path<String>,
    State(state): State<crate::state::State>,
) -> crate::Result<Response<String>> {
    let dict = Dictionary::load(id, &state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_or(Err(StatusCode::NOT_FOUND), |d| Ok(d))?;

    let dict = serde_json::to_string(&dict).map_err(|e| {
        error!("could not serialise dictionary: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Response::new(dict))
}
