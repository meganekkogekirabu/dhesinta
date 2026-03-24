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

use axum::extract::{Path, State};
use axum::http::Response;
use axum::{Form, http::StatusCode};
use axum_login::AuthSession;
use log::error;

use crate::error::Error;
use crate::user::{Credentials, Registration, User};

pub async fn login(
    mut auth_session: AuthSession<crate::state::State>,
    Form(creds): Form<Credentials>,
) -> StatusCode {
    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return StatusCode::UNAUTHORIZED;
        }
        Err(e) => {
            error!("error authenticating: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    if let Err(e) = auth_session.login(&user).await {
        error!("error logging in: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::OK
    }
}

pub async fn logout(mut auth_session: AuthSession<crate::state::State>) -> Result<(), StatusCode> {
    match auth_session.logout().await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("error logging out: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn register(
    State(state): State<crate::state::State>,
    Form(payload): Form<Registration>,
) -> StatusCode {
    match User::register(payload, state.config.secret_key, &state.db).await {
        Err(Error::Database(e)) => {
            if e.as_database_error().unwrap().is_unique_violation() {
                StatusCode::CONFLICT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Ok(_) => StatusCode::CREATED,
    }
}

pub async fn get(
    Path(id): Path<String>,
    State(state): State<crate::state::State>,
) -> Result<Response<String>, StatusCode> {
    let user = User::load(id, &state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = user.ok_or(StatusCode::NOT_FOUND)?;

    let user = serde_json::to_string(&user).map_err(|e| {
        error!("could not serialise user: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Response::new(user))
}

pub async fn whoami(
    auth_session: AuthSession<crate::state::State>,
) -> Result<Response<String>, StatusCode> {
    let user = &auth_session.user.unwrap(); // We should have a guarantee from login_required! that user is not None.

    let user = serde_json::to_string(&user).map_err(|e| {
        error!("could not serialise user: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Response::new(user))
}
