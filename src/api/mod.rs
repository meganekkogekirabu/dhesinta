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

use axum::Router;
use axum::routing::{delete, get, post};
use axum_login::login_required;

use crate::dictionary::Dictionary;
use crate::entry::Entry;
use crate::luau::Module;
use crate::state::State;
use crate::user::User;

mod dictionary;
mod entry;
mod luau;
mod model;
mod user;

use model::HttpModel;

pub fn make() -> Router<State> {
    Router::new()
        // protected
        .route("/dictionaries", post(Dictionary::http_post))
        .route("/entries", post(Entry::http_post))
        .route("/modules", post(Module::http_post))
        .route("/dictionaries/{id}", delete(Dictionary::http_delete))
        .route("/entries/{id}", delete(Entry::http_delete))
        .route("/modules/{id}", delete(Module::http_delete))
        .route("/logout", get(user::logout))
        .route("/users/me", get(user::whoami))
        .route_layer(login_required!(State))
        // unprotected
        .route("/dictionaries/{id}", get(Dictionary::http_get))
        .route("/entries/{id}", get(Entry::http_get))
        .route("/modules/{id}", get(Module::http_get))
        .route("/login", post(user::login))
        .route("/users", post(User::http_post))
        .route("/users/{id}", get(User::http_get))
}
