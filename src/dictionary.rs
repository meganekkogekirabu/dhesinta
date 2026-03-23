use chrono::{DateTime, Utc};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::Nanoid;

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
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

#[derive(Debug, Default, FromRow, Serialize)]
pub struct Dictionary {
    pub id: Nanoid,
    pub owner_id: Nanoid,
    pub name: String,
    pub description: Option<String>,
    pub visibility: DictionaryVisibility,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl crate::Database for Dictionary {
    async fn write(self, db: &SqlitePool) -> anyhow::Result<()> {
        let visibility = self.visibility.as_str();
        let created_at = self.created_at.to_rfc3339();
        let updated_at = self.updated_at.to_rfc3339();
        let Nanoid(id) = self.id;
        let Nanoid(owner_id) = self.owner_id;

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
        .execute(db)
        .await
        .map_err(|e| {
            error!("error creating dictionary {}: {}", id, e);
            e
        })?;

        Ok(())
    }

    async fn load(id: String, db: &SqlitePool) -> anyhow::Result<Option<Self>> {
        debug!("attempting to load dictionary {id}");

        let dict = sqlx::query_as("select * from dictionaries where id = ?;")
            .bind(id)
            .fetch_optional(db)
            .await?;

        Ok(dict)
    }
}
