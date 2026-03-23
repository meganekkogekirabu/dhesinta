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
