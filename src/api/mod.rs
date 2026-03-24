use axum::Router;
use axum::routing::{delete, get, post};
use axum_login::login_required;

use crate::dictionary::Dictionary;
use crate::entry::Entry;
use crate::state::State;
use crate::user::User;

mod dictionary;
mod entry;
mod model;
mod user;

use model::HttpModel;

pub fn make() -> Router<State> {
    Router::new()
        // protected
        .route("/dictionaries", post(Dictionary::http_post))
        .route("/entries", post(Entry::http_post))
        .route("/dictionaries/{id}", delete(Dictionary::http_delete))
        .route("/entries/{id}", delete(Entry::http_delete))
        .route("/logout", get(user::logout))
        .route("/users/me", get(user::whoami))
        .route_layer(login_required!(State))
        // unprotected
        .route("/dictionaries/{id}", get(Dictionary::http_get))
        .route("/entries/{id}", get(Dictionary::http_get))
        .route("/login", post(user::login))
        .route("/users", post(User::http_post))
        .route("/users/{id}", get(User::http_get))
}
