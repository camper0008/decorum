use salvo::{
    oapi::extract::JsonBody,
    prelude::{Extractible, ToSchema},
    session::{Session, SessionDepotExt},
    Depot,
};
use serde::Deserialize;

use crate::{
    api::response::{MessageResponse, ResponseResult},
    db::database::DatabaseParam,
};

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    username: String,
    password: String,
}

#[salvo::endpoint(status_codes(200, 400, 500))]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> ResponseResult {
    let JsonBody(RouteRequest { username, password }) = request;

    if username.trim().is_empty() || password.trim().is_empty() {
        return Err(MessageResponse::bad_request("invalid username or password"));
    }

    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to get database from depot: {err:?}"))
        .map_err(|()| MessageResponse::internal_server_error("internal server error"))?;

    let user = {
        let db = db.read().await;
        let user = db
            .user_from_username(&username)
            .await
            .map_err(|err| log::error!("unable to read username from db: {err:?}"))
            .map_err(|()| MessageResponse::internal_server_error("internal server error"))?;
        let user =
            user.ok_or_else(|| MessageResponse::bad_request("invalid username or password"))?;
        user
    };
    let is_valid = bcrypt::verify(&password, &user.password.0)
        .map_err(|err| log::error!("unable to verify with bcrypt: {err:?}"))
        .map_err(|()| MessageResponse::internal_server_error("internal server error"))?;

    if !is_valid {
        return Err(MessageResponse::bad_request("invalid username or password"));
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
        .map_err(|()| MessageResponse::internal_server_error("internal server error"))?;
    depot.set_session(session);

    Ok(MessageResponse::ok("success"))
}
