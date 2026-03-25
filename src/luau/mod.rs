use async_trait::async_trait;
use log::{debug, error};
use mlua::{FromLua, IntoLua, Lua, Value};
use serde::Serialize;
use sqlx::FromRow;
use std::path::PathBuf;
use std::sync::Arc;

use crate::Nanoid;
use crate::entry::{Entry, Field};

impl IntoLua for Field {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("key", self.key)?;
        table.set("value", self.value)?;
        Ok(Value::Table(table))
    }
}

impl FromLua for Nanoid {
    fn from_lua(value: mlua::Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::String(s) => Ok(Self(s.to_str()?.to_string())),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Nanoid".into(),
                message: Some("expected a string".to_string()),
            }),
        }
    }
}

impl FromLua for Field {
    fn from_lua(value: mlua::Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::Table(table) => {
                let key = table.get("key")?;
                let value = table.get("value")?;
                let entry_id = table.get("entry_id")?;

                Ok(Self {
                    entry_id,
                    key,
                    value,
                })
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Field".into(),
                message: Some("expected a table".to_string()),
            }),
        }
    }
}

impl Entry {
    pub async fn make_fields(mut self, module: String) -> crate::Result<Self> {
        let lua = Lua::new();

        let prelude = PathBuf::from("src/luau/lib/prelude.luau");
        lua.load(prelude).exec()?;
        lua.sandbox(true)?;

        let fields = &self.fields;
        let fields = fields.iter().map(|f| f.clone().into_lua(&lua).unwrap());
        let fields = lua.create_sequence_from(fields)?;
        let fields: Value = lua.load(module).call(fields)?;

        match fields {
            Value::Table(table) => {
                let fields = table
                    .sequence_values::<Field>()
                    .collect::<mlua::Result<Vec<Field>>>()?;
                self.fields = Arc::new(fields);
                Ok(self)
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: fields.type_name(),
                to: "Table".into(),
                message: Some("expected a table".to_string()),
            }
            .into()),
        }
    }
}

#[derive(Default, FromRow, Serialize)]
pub struct Module {
    pub id: Nanoid,
    pub owner_id: Nanoid,
    pub content: String,
}

#[async_trait]
impl crate::Database for Module {
    fn owner(self) -> Nanoid {
        self.owner_id
    }

    async fn write(&self, state: &mut crate::state::State) -> crate::Result<()> {
        let mut tx = state.db.begin().await?;

        let Nanoid(id) = &self.id;
        let Nanoid(owner_id) = &self.owner_id;

        debug!("attempting to create new module {id}");

        sqlx::query!(
            "insert into modules (
                id, owner_id, content
            ) values (?, ?, ?);",
            id,
            owner_id,
            self.content,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!("error creating module {id}: {e}");
            e
        })?;

        Ok(())
    }

    async fn load(id: String, state: &mut crate::state::State) -> crate::Result<Option<Self>> {
        debug!("attempting to load module {id}");

        let module = sqlx::query_as("select * from modules where id = ?;")
            .bind(id)
            .fetch_optional(&state.db)
            .await?;

        Ok(module)
    }

    async fn delete(id: String, state: &mut crate::state::State) -> crate::Result<()> {
        debug!("attempting to delete module {id}");

        let mut tx = state.db.begin().await?;

        sqlx::query!("delete from modules where id = ?;", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }
}
