use salvo::{
    oapi::extract::JsonBody,
    prelude::{Extractible, ToSchema},
    session::SessionDepotExt,
    Depot,
};
use serde::Deserialize;

use crate::{
    api::response::{message_response, MessageResponseResult},
    db::{
        database::{DatabaseParam, EditUser},
        models::{Id, Name},
    },
    password::{HashedPassword, Password, PasswordError},
};

type RequestNickname = Option<String>;
type RequestAvatarId = Option<String>;
type RequestPassword = String;

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    nickname: Option<RequestNickname>,
    avatar_id: Option<RequestAvatarId>,
    password: Option<RequestPassword>,
}

#[salvo::endpoint(status_codes(200, 400, 500))]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> MessageResponseResult {
    let JsonBody(RouteRequest {
        nickname,
        avatar_id,
        password,
    }) = request;

    let nickname = nickname.map(|value| {
        value.map(|value| {
            Name::try_from(value).map_err(|_| message_response::bad_request("invalid nickname"))
        })
    });

    let nickname = match nickname {
        Some(Some(result)) => Some(Some(result?)),
        Some(None) => Some(None),
        None => None,
    };

    let avatar_id = avatar_id.map(|value| {
        value.map(|value| {
            Id::try_from(value).map_err(|_| message_response::bad_request("invalid avatar id"))
        })
    });

    let avatar_id = match avatar_id {
        Some(Some(result)) => Some(Some(result?)),
        Some(None) => Some(None),
        None => None,
    };

    let password = password
        .map(|password| {
            Password::try_from(password).map_err(|err| match err {
                PasswordError::TooShort(_) => "invalid password: too short",
                PasswordError::TooLong(_) => "invalid password: too long",
                PasswordError::InvalidCharacters => "invalid password: invalid characters",
            })
        })
        .map(|password| {
            password.and_then(|password| {
                HashedPassword::try_from(password).map_err(|_| "invalid password")
            })
        })
        .map(|password| password.map_err(message_response::bad_request));

    let password = match password {
        Some(result) => Some(result?),
        None => None,
    };

    let user_id = depot
        .session()
        .and_then(|session| session.get::<Id>("user_id"))
        .ok_or_else(|| message_response::unauthorized("invalid session"))?;

    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to get database from depot: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;

    let user = {
        let db = db.read().await;
        let user = db
            .user_from_id(&user_id)
            .await
            .map_err(|err| log::error!("unable to read id from db: {err:?}"))
            .map_err(|()| message_response::internal_server_error("internal server error"))?;
        user.ok_or_else(|| message_response::bad_request("invalid session"))?
    };

    {
        let mut db = db.write().await;
        db.edit_user(EditUser {
            id: user.id,
            avatar_id: avatar_id.unwrap_or(user.avatar_id),
            nickname: nickname.unwrap_or(user.nickname),
            password: password.unwrap_or(user.password),
            permission: user.permission,
            deleted: user.deleted,
        })
        .await
        .map_err(|err| log::error!("unable to edit user: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;
    }

    Ok(message_response::ok("success"))
}
