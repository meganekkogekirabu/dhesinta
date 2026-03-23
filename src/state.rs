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

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct State {
    pub db: SqlitePool,
}

impl State {
    pub fn new(db_url: String) -> anyhow::Result<Self> {
        let opts = SqliteConnectOptions::from_str(&db_url)?;
        let db = SqlitePool::connect_lazy_with(opts);
        Ok(Self { db })
    }
}
