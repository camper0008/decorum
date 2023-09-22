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
        models::{Name, Permission},
    },
    password::{Password, PasswordError},
};

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    username: String,
    password: String,
}

#[salvo::endpoint(status_codes(201, 400, 500))]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> ResponseResult {
    let JsonBody(RouteRequest { username, password }) = request;

    let username: Name = username
        .try_into()
        .map_err(|err| MessageResponse::bad_request(err))?;

    let password: Password = password.try_into().map_err(|err| {
        MessageResponse::bad_request(match err {
            PasswordError::TooShort(_) => "invalid password: too short",
            PasswordError::TooLong(_) => "invalid password: too long",
            PasswordError::InvalidCharacters => "invalid password: invalid characters",
        })
    })?;

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
    let password = bcrypt::hash::<String>(password.into(), bcrypt::DEFAULT_COST)
        .map_err(|err| log::error!("unable to hash pw: {err:?}"))
        .map_err(|()| MessageResponse::internal_server_error("internal server error"))?;
    {
        let mut db = db.write().await;

        let username = username
            .try_into()
            .map_err(|_| MessageResponse::bad_request("invalid username length"))?;
        let password = password
            .try_into()
            .map_err(|_| MessageResponse::bad_request("invalid password length"))?;

        db.create_user(CreateUser {
            username,
            nickname: None,
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
