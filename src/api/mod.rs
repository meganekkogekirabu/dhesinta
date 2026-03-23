use axum::Router;
use axum::routing::{get, post};
use axum_login::login_required;

use crate::state::State;

mod dictionary;
mod entry;
mod user;

pub fn make() -> Router<State> {
    Router::new()
        // protected
        .route("/dictionaries", post(dictionary::create))
        .route("/entries", post(entry::create))
        .route("/logout", get(user::logout))
        .route_layer(login_required!(State))
        // unprotected
        .route("/dictionaries/{id}", get(dictionary::get))
        .route("/entries/{id}", get(entry::get))
        .route("/login", post(user::login))
        .route("/users", post(user::register))
        .route("/users/{id}", get(user::get))
}
