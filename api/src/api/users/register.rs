use salvo::{
    oapi::extract::JsonBody,
    prelude::{Extractible, ToSchema},
    Depot,
};
use serde::Deserialize;

use crate::{
    api::response::{MessageResponse, ResponseResult},
    db::{
        database::{CreateUser, DatabaseParam},
        models::Permission,
    },
};

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    username: String,
    password: String,
}

#[salvo::endpoint(status_codes(201, 400, 500))]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> ResponseResult {
    let JsonBody(RouteRequest { username, password }) = request;

    if username.trim().is_empty() || password.trim().is_empty() {
        return Err(MessageResponse::bad_request("invalid username or password"));
    }

    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to obtain database from depot: {err:?}"))
        .map_err(|()| MessageResponse::internal_server_error("internal server error"))?;

    {
        let db = db.read().await;
        let user = db.user_from_username(&username).await.map_err(|err| {
            log::error!("unable to read username from db: {err:?}");
            MessageResponse::internal_server_error("internal server error")
        })?;

        if user.is_some() {
            return Err(MessageResponse::bad_request("user already exists"));
        }
    }
    log::info!("b={password}");
    let password = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|err| log::error!("unable to hash pw: {err:?}"))
        .map_err(|()| MessageResponse::internal_server_error("internal server error"))?;
    log::info!("a={password}");
    {
        let mut db = db.write().await;
        let nickname = username.clone();
        db.create_user(CreateUser {
            username,
            nickname,
            password,
            permission: Permission::Unverified,
            avatar_id: None,
        })
        .await
        .map_err(|err| log::error!("unable to save user in db: {err:?}"))
        .map_err(|()| MessageResponse::internal_server_error("error creating post"))?;
    }

    Ok(MessageResponse::created("user created"))
}
