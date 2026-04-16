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
use axum::extract::Path;
use axum::http::Response;
use axum::{http::StatusCode, Form, Router};
use axum_login::{login_required, AuthSession};
use log::error;
use serde::Deserialize;
use std::sync::Arc;
use axum::routing::{delete, get, post};
use crate::api::model::HttpModel;
use crate::error::Error;
use crate::database::{Credentials, User};
use crate::Nanoid;
use crate::database::model::DatabaseModel;

impl User {
    pub async fn login(
        mut session: AuthSession<crate::state::State>,
        Form(creds): Form<Credentials>,
    ) -> StatusCode {
        let user = match session.authenticate(creds).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return StatusCode::UNAUTHORIZED;
            }
            Err(e) => {
                error!("error authenticating: {e}");
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        };

        if let Err(e) = session.login(&user).await {
            error!("error logging in: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        } else {
            StatusCode::OK
        }
    }

    pub async fn logout(mut session: AuthSession<crate::state::State>) -> Result<(), StatusCode> {
        match session.logout().await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("error logging out: {e}");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    pub async fn whoami(
        session: AuthSession<crate::state::State>,
    ) -> Result<Response<String>, StatusCode> {
        let user = &session.user.unwrap(); // We should have a guarantee from login_required! that user is not None.

        let user = serde_json::to_string(&user).map_err(|e| {
            error!("could not serialise user: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(Response::new(user))
    }
}

#[derive(Deserialize)]
pub struct Registration {
    email: String,
    username: String,
    password: String,
}

#[async_trait]
impl HttpModel for User {
    async fn get(
        Path(id): Path<String>,
        mut session: AuthSession<crate::state::State>,
    ) -> Result<Response<String>, StatusCode> {
        let user = User::database_get(id, &mut session.backend)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let user = user.ok_or(StatusCode::NOT_FOUND)?;

        let user = serde_json::to_string(&user).map_err(|e| {
            error!("could not serialise user: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(Response::new(user))
    }

    type Payload = Form<Registration>;

    async fn create(
        mut session: AuthSession<crate::state::State>,
        Form(payload): Self::Payload,
    ) -> Result<Response<String>, StatusCode> {
        let user = User {
            username: payload.username,
            password: payload.password,
            email: payload.email,
            id: Nanoid(nanoid::nanoid!()),
        };

        let user = Arc::new(user);

        match user.clone().database_write(&mut session.backend).await {
            Err(Error::Database(e)) => {
                if e.as_database_error().unwrap().is_unique_violation() {
                    Err(StatusCode::CONFLICT)
                } else {
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            Ok(_) => match serde_json::to_string(&user) {
                Ok(user) => {
                    let mut response = Response::new(user);
                    let status = response.status_mut();
                    *status = StatusCode::CREATED;
                    Ok(response)
                }
                Err(e) => {
                    error!("error serialising user: {e}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            },
        }
    }

    fn make() -> Router<crate::state::State> {
        Router::new()
            .route("/", delete(Self::delete))
            .route("/logout", get(Self::logout))
            .route_layer(login_required!(crate::state::State))
            .route("/", post(Self::create))
            .route("/{id}", get(Self::get))
            .route("/login", post(Self::login))
            .route("/me", get(Self::whoami))
    }
}
