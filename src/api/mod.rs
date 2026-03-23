use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

use crate::state;

mod dictionary;
mod entry;

pub fn make(state: Arc<state::State>) -> Router<Arc<state::State>> {
    Router::new()
        .route("/dictionaries", post(dictionary::create))
        .route("/dictionaries/{id}", get(dictionary::get))
        .route("/entries", post(entry::create))
        .route("/entries/{id}", get(entry::get))
        .with_state(state)
}
