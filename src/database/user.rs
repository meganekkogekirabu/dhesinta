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

use argonautica::{Hasher, Verifier};
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::Nanoid;

#[derive(Clone, FromRow, Serialize)]
pub struct User {
    pub id: Nanoid,
    pub email: String,
    pub username: String,

    #[serde(skip)]
    pub password: String,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("email", &self.email)
            .field("username", &self.username)
            .finish()
    }
}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        let Nanoid(id) = &self.id;
        id.to_string()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    email: String,
    password: String,
}

impl AuthnBackend for crate::state::State {
    type Error = sqlx::Error;
    type Credentials = Credentials;
    type User = User;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = sqlx::query_as("select * from users where email = ?;")
            .bind(creds.email)
            .fetch_optional(&self.db)
            .await?;

        Ok(user.filter(|user| {
            let mut verifier = Verifier::new();
            verifier
                .with_hash(&user.password)
                .with_password(&creds.password)
                .with_secret_key(&self.config.secret_key)
                .verify()
                .unwrap()
        }))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as("select * from users where id = ?;")
            .bind(user_id)
            .fetch_optional(&self.db)
            .await?;

        Ok(user)
    }
}

#[async_trait]
impl crate::database::model::DatabaseModel for User {
    type Query = ();

    fn owner(self) -> Nanoid {
        self.id
    }

    async fn database_write(&self, state: &mut crate::state::State) -> crate::Result<()> {
        let mut tx = state.db.begin().await?;

        let mut hasher = Hasher::new();
        let hash = hasher
            .with_password(&self.password)
            .with_secret_key(state.config.secret_key.clone())
            .hash()
            .map_err(|e| {
                error!("failed to hash password: {e}");
                e
            })?;

        sqlx::query!(
            "insert into users (id, email, password, username) values (?, ?, ?, ?);",
            self.id,
            self.email,
            hash,
            self.username
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!("error creating user: {e}");
            e
        })?;

        tx.commit().await?;

        Ok(())
    }

    async fn database_delete(id: String, state: &mut crate::state::State) -> crate::Result<()> {
        debug!("attempting to delete user {id}");

        let mut tx = state.db.begin().await?;

        sqlx::query!("delete from users where id = ?;", id)
            .execute(&mut *tx)
            .await?;

        Ok(())
    }

    async fn database_get(
        id: String,
        state: &mut crate::state::State,
    ) -> crate::Result<Option<Self>> {
        debug!("attempting to load user {id}");

        let user = sqlx::query_as("select * from users where id = ?;")
            .bind(id)
            .fetch_optional(&state.db)
            .await?;

        Ok(user)
    }
}
