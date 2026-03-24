use async_trait::async_trait;
use axum::extract::Path;
use axum::http::{Response, StatusCode};
use axum_login::AuthSession;
use log::error;
use serde::Serialize;

#[async_trait]
pub trait HttpModel: crate::Database + Serialize {
    async fn http_get(
        Path(id): Path<String>,
        mut session: AuthSession<crate::state::State>,
    ) -> Result<Response<String>, StatusCode> {
        match Self::load(id, &mut session.backend).await {
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

    async fn http_delete(
        Path(id): Path<String>,
        mut session: AuthSession<crate::state::State>,
    ) -> StatusCode {
        match Self::load(id.to_owned(), &mut session.backend).await {
            Ok(Some(model)) => {
                if model.owner() != session.user.unwrap().id {
                    return StatusCode::UNAUTHORIZED;
                }
            }
            Ok(None) => return StatusCode::NOT_FOUND,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
        }

        if let Err(_) = Self::delete(id, &mut session.backend).await {
            StatusCode::INTERNAL_SERVER_ERROR
        } else {
            StatusCode::NO_CONTENT
        }
    }

    // Creation is more complicated so leave it to the implementer.

    type Payload;

    async fn http_post(
        session: AuthSession<crate::state::State>,
        payload: Self::Payload,
    ) -> Result<Response<String>, StatusCode>;
}
