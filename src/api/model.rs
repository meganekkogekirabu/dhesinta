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
use axum::extract::{Path, Query};
use axum::http::{Response, StatusCode};
use axum::Router;
use axum_login::AuthSession;
use log::error;
use serde::Serialize;

#[async_trait]
pub trait HttpModel: crate::Database + Serialize {
    async fn get(
        Path(id): Path<String>,
        mut session: AuthSession<crate::state::State>,
    ) -> Result<Response<String>, StatusCode> {
        match Self::database_get(id, &mut session.backend).await {
            Ok(Some(model)) => match serde_json::to_string(&model) {
                Ok(model) => Ok(Response::new(model)),
                Err(e) => {
                    error!("could not serialise model: {e}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            },
            Ok(None) => Err(StatusCode::NOT_FOUND),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn query(
        Query(query): Query<<Self as crate::Database>::Query>,
        mut session: AuthSession<crate::state::State>,
    ) -> Result<Response<String>, StatusCode> {
        match Self::database_query(query, &mut session.backend).await {
            Ok(all) => match serde_json::to_string(&all) {
                Ok(all) => Ok(Response::new(all)),
                Err(e) => {
                    error!("could not serialise query response: {e}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            },
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn delete(
        Path(id): Path<String>,
        mut session: AuthSession<crate::state::State>,
    ) -> StatusCode {
        match Self::database_get(id.to_owned(), &mut session.backend).await {
            Ok(Some(model)) => {
                if model.owner() != session.user.unwrap().id {
                    return StatusCode::UNAUTHORIZED;
                }
            }
            Ok(None) => return StatusCode::NOT_FOUND,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        }

        if let Err(_) = Self::database_delete(id, &mut session.backend).await {
            StatusCode::INTERNAL_SERVER_ERROR
        } else {
            StatusCode::NO_CONTENT
        }
    }

    // Creation is more complicated so leave it to the implementer.

    type Payload;

    async fn create(
        session: AuthSession<crate::state::State>,
        payload: Self::Payload,
    ) -> Result<Response<String>, StatusCode>;

    fn make() -> Router<crate::state::State>;
}
