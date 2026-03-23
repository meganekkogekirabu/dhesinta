use axum::extract::{Path, State};
use axum::http::Response;
use axum::{Form, http::StatusCode};
use axum_login::AuthSession;
use log::error;

use crate::user::{Credentials, Registration, User};

pub async fn login(
    mut auth_session: AuthSession<crate::state::State>,
    Form(creds): Form<Credentials>,
) -> crate::Result<()> {
    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(e) => {
            error!("error authenticating: {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if let Err(err) = auth_session.login(&user).await {
        error!("error logging in: {err}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(())
}

pub async fn logout(mut auth_session: AuthSession<crate::state::State>) -> crate::Result<()> {
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
) -> crate::Result<StatusCode> {
    User::register(payload, state.config.secret_key, &state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

pub async fn get(
    Path(id): Path<String>,
    State(state): State<crate::state::State>,
) -> crate::Result<Response<String>> {
    let user = User::load(id, &state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_or(Err(StatusCode::NOT_FOUND), |d| Ok(d))?;

    let user = serde_json::to_string(&user).map_err(|e| {
        error!("could not serialise user: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Response::new(user))
}
