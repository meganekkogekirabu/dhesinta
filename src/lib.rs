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

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod api;
pub mod config;
pub mod dictionary;
pub mod entry;
pub mod error;
pub mod state;
pub mod user;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Nanoid(pub String);

impl Default for Nanoid {
    fn default() -> Self {
        Self(nanoid::nanoid!())
    }
}

#[async_trait]
pub trait Database: Sized {
    type Query: Send + for<'a> Deserialize<'a>;

    fn owner(self) -> Nanoid;

    async fn database_write(&self, state: &mut crate::state::State) -> crate::Result<()>;

    async fn database_get(
        id: String,
        state: &mut crate::state::State,
    ) -> crate::Result<Option<Self>>;

    // stupid hack of an implementation, but we don't need database_get_all for users
    #[allow(unused_variables)]
    async fn database_query(
        query: Self::Query,
        state: &mut crate::state::State,
    ) -> crate::Result<Vec<Self>> {
        Ok(vec![])
    }

    async fn database_delete(id: String, state: &mut crate::state::State) -> crate::Result<()>;
}
