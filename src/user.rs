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

use anyhow::anyhow;
use argonautica::{Hasher, Verifier};
use axum_login::{AuthUser, AuthnBackend, UserId};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

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
            .field("password", &"<redacted>")
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

#[derive(Deserialize)]
pub struct Registration {
    email: String,
    username: String,
    password: String,
}

impl User {
    pub async fn register(
        regi: Registration,
        secret_key: String,
        db: &SqlitePool,
    ) -> anyhow::Result<()> {
        let mut tx = db.begin().await?;

        let mut hasher = Hasher::new();
        let hash = hasher
            .with_password(regi.password)
            .with_secret_key(secret_key)
            .hash()
            .map_err(|e| anyhow!("failed to hash password: {e}"))?;

        let id = nanoid::nanoid!();

        sqlx::query!(
            "insert into users (id, email, password, username) values (?, ?, ?, ?);",
            id,
            regi.email,
            hash,
            regi.username
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            // TODO: This can happen for non-unique emails and usernames
            // so we should send back a 409 Conflict in that case.
            error!("error creating user: {e}");
            e
        })?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn load(id: String, db: &SqlitePool) -> anyhow::Result<Option<Self>> {
        debug!("attempting to load user {id}");

        let user = sqlx::query_as("select * from users where id = ?;")
            .bind(id)
            .fetch_optional(db)
            .await?;

        Ok(user)
    }
}
