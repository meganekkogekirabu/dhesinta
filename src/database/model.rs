use async_trait::async_trait;
use serde::Deserialize;
use crate::{state, Nanoid};

#[async_trait]
pub trait DatabaseModel: Sized {
    type Query: Send + for<'a> Deserialize<'a>;

    fn owner(self) -> Nanoid;

    async fn database_write(&self, state: &mut state::State) -> crate::Result<()>;

    async fn database_get(
        id: String,
        state: &mut state::State,
    ) -> crate::Result<Option<Self>>;

    // stupid hack of an implementation, but we don't need database_get_all for users
    #[allow(unused_variables)]
    async fn database_query(
        query: Self::Query,
        state: &mut state::State,
    ) -> crate::Result<Vec<Self>> {
        Ok(vec![])
    }

    async fn database_delete(id: String, state: &mut state::State) -> crate::Result<()>;
}