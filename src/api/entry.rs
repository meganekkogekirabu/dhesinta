use axum::extract::{Json, Path, State};
use axum::http::{Response, StatusCode};
use chrono::Utc;
use condict::entry::{Entry, Field};
use condict::{Database, Nanoid};
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
    State(state): condict::State,
    Json(payload): Json<CreateEntryPayload>,
) -> condict::Result<StatusCode> {
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
        dictionary_id,
        word,
        fields,
        created_at: now,
        updated_at: now,
        ..Default::default()
    };

    match entry.write(&state.db).await {
        Ok(()) => Ok(StatusCode::CREATED),
        Err(e) => match e.downcast().unwrap() {
            sqlx::Error::Database(_) => Err(StatusCode::BAD_REQUEST),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
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
    State(state): condict::State,
) -> condict::Result<Response<String>> {
    let entry = Entry::load(id, &state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_or(Err(StatusCode::NOT_FOUND), |e| Ok(e))?;

    let fields = entry.fields.clone();
    let fields: HashMap<_, _> = fields.iter().map(|f| (&f.key, &f.value)).collect();

    let entry = GetEntryResponse { fields, entry };

    let entry = serde_json::to_string(&entry).map_err(|e| {
        error!("could not serialise entry: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Response::new(entry))
}
