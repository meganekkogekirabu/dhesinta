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
use chrono::{DateTime, Utc};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::Nanoid;

#[derive(Clone, Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "text")]
#[sqlx(rename_all = "snake_case")]
pub enum DictionaryVisibility {
    #[default]
    Public,
    Private,
    Unlisted,
}

impl DictionaryVisibility {
    fn as_str(&self) -> &str {
        match self {
            Self::Public => "public",
            Self::Private => "private",
            Self::Unlisted => "unlisted",
        }
    }
}

#[derive(Clone, Debug, Default, FromRow, Serialize)]
pub struct Dictionary {
    pub id: Nanoid,
    pub owner_id: Nanoid,
    pub name: String,
    pub description: Option<String>,
    pub visibility: DictionaryVisibility,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
impl crate::Database for Dictionary {
    fn owner(self) -> Nanoid {
        self.owner_id
    }

    async fn write(&self, state: &mut crate::state::State) -> crate::Result<()> {
        let mut tx = state.db.begin().await?;

        let visibility = self.visibility.as_str();
        let created_at = self.created_at.to_rfc3339();
        let updated_at = self.updated_at.to_rfc3339();
        let Nanoid(id) = &self.id;
        let Nanoid(owner_id) = &self.owner_id;

        debug!("attempting to create new dictionary {id}");

        sqlx::query!(
            "insert into dictionaries (
                id, owner_id, name, description, visibility, created_at, updated_at
            ) values (?, ?, ?, ?, ?, ?, ?);",
            id,
            owner_id,
            self.name,
            self.description,
            visibility,
            created_at,
            updated_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!("error creating dictionary {}: {}", id, e);
            e
        })?;

        Ok(())
    }

    async fn load(id: String, state: &mut crate::state::State) -> crate::Result<Option<Self>> {
        debug!("attempting to load dictionary {id}");

        let dict = sqlx::query_as("select * from dictionaries where id = ?;")
            .bind(id)
            .fetch_optional(&state.db)
            .await?;

        Ok(dict)
    }

    async fn delete(id: String, state: &mut crate::state::State) -> crate::Result<()> {
        debug!("attempting to delete dictionary {id}");

        let mut tx = state.db.begin().await?;

        sqlx::query!("delete from dictionaries where id = ?;", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }
}
