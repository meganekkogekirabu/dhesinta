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

use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;

pub mod dictionary;
pub mod entry;
pub mod state;

pub type Result<T> = std::result::Result<T, StatusCode>;
pub type State = axum::extract::State<Arc<state::State>>;

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Nanoid(pub String);

impl Default for Nanoid {
    fn default() -> Self {
        Self(nanoid::nanoid!())
    }
}

pub trait Database: Sized {
    fn write(self, db: &SqlitePool) -> impl std::future::Future<Output = anyhow::Result<()>>;

    fn load(
        id: String,
        db: &SqlitePool,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Self>>>;
}
