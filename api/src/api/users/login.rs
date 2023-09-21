use salvo::{
    http::errors::StatusResult,
    oapi::extract::JsonBody,
    prelude::{Extractible, StatusCode, StatusError, ToSchema},
    session::{Session, SessionDepotExt},
    Depot,
};
use serde::Deserialize;

use crate::db::database::DatabaseParam;

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    username: String,
    password: String,
}

#[salvo::endpoint(status_codes(200, 400, 500))]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> StatusResult<StatusCode> {
    let JsonBody(RouteRequest { username, password }) = request;

    if username.trim().is_empty() || password.trim().is_empty() {
        return Err(StatusError::bad_request().brief("invalid username or password"));
    }

    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to get database from depot: {err:?}"))
        .map_err(|()| StatusError::internal_server_error().brief("internal server error"))?;

    let user = {
        let db = db.read().await;
        let user = db
            .user_from_username(&username)
            .await
            .map_err(|err| log::error!("unable to read username from db: {err:?}"))
            .map_err(|()| StatusError::internal_server_error().brief("internal server error"))?;
        let user =
            user.ok_or_else(|| StatusError::bad_request().brief("invalid username or password"))?;
        user
    };
    let is_valid = bcrypt::verify(&password, &user.password.0)
        .map_err(|err| log::error!("unable to verify with bcrypt: {err:?}"))
        .map_err(|()| StatusError::internal_server_error().brief("internal server error"))?;

    if !is_valid {
        return Err(StatusError::bad_request().brief("invalid username or password"));
    }

    let mut session = Session::new();
    session
        .insert("user_id", &user.id.0)
        .map_err(|err| {
            log::error!(
                "unable to insert user session for user {}: {err:?}",
                user.id
            )
        })
        .map_err(|()| StatusError::internal_server_error().brief("internal server error"))?;
    depot.set_session(session);

    Ok(StatusCode::OK)
}
