use salvo::{
    oapi::extract::JsonBody,
    prelude::{Extractible, ToSchema},
    session::{Session, SessionDepotExt},
    Depot,
};
use serde::Deserialize;

use crate::{
    api::response::{message_response, MessageResponseResult},
    db::{database::DatabaseParam, models::Name},
    password::Password,
};

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    username: String,
    password: String,
}

#[salvo::endpoint(status_codes(200, 400, 500))]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> MessageResponseResult {
    let JsonBody(RouteRequest { username, password }) = request;

    let username =
        Name::try_from(username).map_err(|_| message_response::bad_request("invalid username"))?;

    let password = Password::try_from(password)
        .map_err(|_| message_response::bad_request("invalid password"))?;

    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to get database from depot: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;

    let user = {
        let db = db.read().await;
        let user = db
            .user_from_username(&username)
            .await
            .map_err(|err| log::error!("unable to read username from db: {err:?}"))
            .map_err(|()| message_response::internal_server_error("internal server error"))?;
        user.ok_or_else(|| message_response::bad_request("invalid username or password"))?
    };
    let is_valid = bcrypt::verify::<String>(password.into(), (&user.password).into())
        .map_err(|err| log::error!("unable to verify with bcrypt: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;

    if !is_valid {
        return Err(message_response::bad_request(
            "invalid username or password",
        ));
    }

    let mut session = Session::new();
    session
        .insert("user_id", &user.id.to_string())
        .map_err(|err| {
            log::error!(
                "unable to insert user session for user {}: {err:?}",
                user.id
            );
        })
        .map_err(|()| message_response::internal_server_error("internal server error"))?;
    depot.set_session(session);

    Ok(message_response::ok("success"))
}
