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
