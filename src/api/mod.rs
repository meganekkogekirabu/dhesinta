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
