use salvo::{
    oapi::extract::JsonBody,
    prelude::{Extractible, ToSchema},
    Depot,
};
use serde::Deserialize;

use crate::{
    api::response::{message_response, MessageResponseResult},
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
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> MessageResponseResult {
    let JsonBody(RouteRequest { username, password }) = request;

    let username: Name = username.try_into().map_err(message_response::bad_request)?;

    let password: Password = password.try_into().map_err(|err| {
        message_response::bad_request(match err {
            PasswordError::TooShort(_) => "invalid password: too short",
            PasswordError::TooLong(_) => "invalid password: too long",
            PasswordError::InvalidCharacters => "invalid password: invalid characters",
        })
    })?;

    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to obtain database from depot: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;

    {
        let db = db.read().await;
        let user = db.user_from_username(&username).await.map_err(|err| {
            log::error!("unable to read username from db: {err:?}");
            message_response::internal_server_error("internal server error")
        })?;

        if user.is_some() {
            return Err(message_response::bad_request("user already exists"));
        }
    }
    let password = bcrypt::hash::<String>(password.into(), bcrypt::DEFAULT_COST)
        .map_err(|err| log::error!("unable to hash pw: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;
    {
        let mut db = db.write().await;

        let password = password
            .try_into()
            .map_err(|_| message_response::bad_request("invalid password length"))?;

        db.create_user(CreateUser {
            username,
            nickname: None,
            password,
            permission: Permission::default(),
            avatar_id: None,
        })
        .await
        .map_err(|err| log::error!("unable to save user in db: {err:?}"))
        .map_err(|()| message_response::internal_server_error("error creating post"))?;
    }

    Ok(message_response::created("user created"))
}
