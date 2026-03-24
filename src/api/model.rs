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
        let model = Self::load(id, &mut session.backend)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let model = model.ok_or(StatusCode::NOT_FOUND)?;

        let model = serde_json::to_string(&model).map_err(|e| {
            error!("could not serialise model: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(Response::new(model))
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
            StatusCode::OK
        }
    }

    // Creation is more complicated so leave it to the implementer.

    type Payload;

    async fn http_post(
        session: AuthSession<crate::state::State>,
        payload: Self::Payload,
    ) -> StatusCode;
}
