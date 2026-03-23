use axum::Json;
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use chrono::Utc;
use condict::dictionary::{Dictionary, DictionaryVisibility};
use condict::{Database, Nanoid};
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
    State(state): condict::State,
    Json(payload): Json<CreateDictPayload>,
) -> condict::Result<StatusCode> {
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
    State(state): condict::State,
) -> condict::Result<Response<String>> {
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
