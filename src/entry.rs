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

use chrono::{DateTime, Utc};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, QueryBuilder, SqlitePool, SqliteTransaction};

use crate::Nanoid;

#[derive(Debug, Default, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub id: Nanoid,
    pub dictionary_id: Nanoid,
    pub owner_id: Nanoid,
    pub word: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    #[sqlx(skip)]
    #[serde(skip)]
    pub fields: Vec<Field>,
}

impl crate::Database for Entry {
    async fn write(self, db: &SqlitePool) -> crate::Result<()> {
        let mut tx = db.begin().await?;

        let Nanoid(dictionary_id) = self.dictionary_id;
        let Nanoid(owner_id) = self.owner_id;
        let Nanoid(id) = self.id;
        let created_at = self.created_at.to_rfc3339();
        let updated_at = self.updated_at.to_rfc3339();

        debug!("attempting to create new entry on {dictionary_id}");

        sqlx::query!(
            "insert into entries (
                id, owner_id, dictionary_id, word, created_at, updated_at
            ) values (?, ?, ?, ?, ?, ?);",
            id,
            owner_id,
            dictionary_id,
            self.word,
            created_at,
            updated_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!("failed to write entry: {e}");
            e
        })?;

        write_fields(self.fields, tx).await?;

        Ok(())
    }

    async fn load(id: String, db: &SqlitePool) -> crate::Result<Option<Self>> {
        debug!("attempting to load entry {id}");

        let entry = sqlx::query_as("select * from entries where id = ?;")
            .bind(&id)
            .fetch_optional(db)
            .await
            .map_err(|e| {
                error!("error finding entry {id}: {e}");
                e
            })?;

        let fields: Vec<Field> = sqlx::query_as("select * from entry_fields where entry_id = ?;")
            .bind(&id)
            .fetch_all(db)
            .await
            .map_err(|e| {
                error!("error finding fields for entry {id}: {e}");
                e
            })?;

        let entry = entry.map(|mut e: Entry| {
            e.fields = fields;
            e
        });

        Ok(entry)
    }

    async fn delete(id: String, db: &SqlitePool) -> crate::Result<()> {
        debug!("attempting to delete entry {id}");

        let mut tx = db.begin().await?;

        sqlx::query!("delete from entries where id = ?;", id)
            .execute(&mut *tx)
            .await?;

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub entry_id: Nanoid,

    #[sqlx(rename = "field_key")]
    pub key: String,

    #[sqlx(rename = "field_value")]
    pub value: String,
}

impl Field {
    pub fn from_tuple(entry_id: &Nanoid, map: (&String, &String)) -> Self {
        let (key, value) = map;

        let entry_id = entry_id.to_owned();
        let key = key.to_owned();
        let value = value.to_owned();

        Self {
            entry_id,
            key,
            value,
        }
    }
}

const BIND_LIMIT: usize = 65535;

async fn write_fields<'c>(fields: Vec<Field>, mut tx: SqliteTransaction<'c>) -> crate::Result<()> {
    let mut query: QueryBuilder<'_, sqlx::Sqlite> =
        QueryBuilder::new("insert into entry_fields (entry_id, field_key, field_value) ");

    query.push_values(fields.iter().take(BIND_LIMIT / 4), |mut b, field| {
        let Nanoid(entry_id) = &field.entry_id;

        b.push_bind(entry_id)
            .push_bind(&field.key)
            .push_bind(&field.value);
    });

    query.build().execute(&mut *tx).await.map_err(|e| {
        error!("failed to write fields: {e}");
        e
    })?;

    tx.commit().await?;

    Ok(())
}
