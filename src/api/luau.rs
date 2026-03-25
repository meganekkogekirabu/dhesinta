use async_trait::async_trait;
use axum::extract::Json;
use axum::http::{Response, StatusCode};
use axum_login::AuthSession;
use log::error;
use serde::Deserialize;
use std::sync::Arc;

use crate::api::model::HttpModel;
use crate::error::Error;
use crate::luau::Module;
use crate::{Database, Nanoid};

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModulePayload {
    owner_id: Nanoid,
    content: String,
}

#[async_trait]
impl HttpModel for Module {
    type Payload = Json<ModulePayload>;

    async fn http_post(
        mut session: AuthSession<crate::state::State>,
        Json(payload): Self::Payload,
    ) -> Result<Response<String>, StatusCode> {
        let module = Self {
            content: payload.content,
            owner_id: payload.owner_id,
            ..Default::default()
        };

        let module = Arc::new(module);

        match module.clone().write(&mut session.backend).await {
            Ok(_) => match serde_json::to_string(&module) {
                Ok(dictionary) => {
                    let mut response = Response::new(dictionary);
                    let status = response.status_mut();
                    *status = StatusCode::CREATED;
                    Ok(response)
                }
                Err(e) => {
                    error!("error serialising module: {e}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            },
            Err(Error::Database(e)) => {
                if e.as_database_error().unwrap().is_foreign_key_violation() {
                    Err(StatusCode::BAD_REQUEST)
                } else {
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
