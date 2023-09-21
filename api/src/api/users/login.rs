use salvo::{
    http::errors::StatusResult,
    oapi::extract::JsonBody,
    prelude::{Extractible, StatusError, ToSchema},
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

#[salvo::endpoint]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> StatusResult<()> {
    let JsonBody(RouteRequest { username, password }) = request;

    if username.trim().is_empty() || password.trim().is_empty() {
        return Err(StatusError::bad_request().brief("invalid username or password"));
    }

    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|_| StatusError::internal_server_error().brief("internal server error"))?;

    let user = {
        let db = db.read().await;
        let user = db
            .user_from_username(&username)
            .await
            .map_err(|_| StatusError::internal_server_error().brief("internal server error"))?;

        let user =
            user.ok_or_else(|| StatusError::bad_request().brief("invalid username or password"))?;
        user
    };
    let is_valid = bcrypt::verify(&user.password.0, &password)
        .map_err(|_| StatusError::internal_server_error().brief("internal server error"))?;

    if !is_valid {
        return Err(StatusError::bad_request().brief("invalid username or password"));
    }

    let mut session = Session::new();
    session
        .insert("user_id", user.id.0)
        .map_err(|_| StatusError::internal_server_error().brief("internal server error"))?;
    depot.set_session(session);

    Ok(())
}
