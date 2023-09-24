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
        models::{Id, Permission},
    },
    permission_verification::{self, permission_for_important_actions},
};

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    id: Id,
    permission: Permission,
}

#[salvo::endpoint(status_codes(200, 400, 500))]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> MessageResponseResult {
    let JsonBody(RouteRequest { id, permission }) = request;

    let admin_id = depot
        .session()
        .and_then(|session| session.get::<Id>("user_id"))
        .ok_or_else(|| message_response::unauthorized("invalid session"))?;

    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to get database from depot: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;

    {
        let db = db.read().await;
        let user = db
            .user_from_id(&admin_id)
            .await
            .map_err(|err| log::error!("unable to read id from db: {err:?}"))
            .map_err(|()| message_response::internal_server_error("internal server error"))?;
        let user = user.ok_or_else(|| message_response::bad_request("invalid session"))?;
        if !permission_verification::is_allowed(
            &user.permission,
            &permission_for_important_actions(),
        ) || !permission_verification::is_allowed(&user.permission, &permission)
        {
            return Err(message_response::unauthorized("invalid session"));
        }
    };

    let id = Id::try_from(id).map_err(|_| message_response::bad_request("invalid user id"))?;

    let user = {
        let db = db.read().await;
        let user = db
            .user_from_id(&id)
            .await
            .map_err(|err| log::error!("unable to read id from db: {err:?}"))
            .map_err(|()| message_response::internal_server_error("internal server error"))?;
        let user = user.ok_or_else(|| message_response::bad_request("invalid session"))?;
        user
    };

    {
        let mut db = db.write().await;
        db.edit_user(EditUser {
            id,
            avatar_id: user.avatar_id,
            nickname: user.nickname,
            password: user.password,
            permission,
            deleted: user.deleted,
        })
        .await
        .map_err(|err| log::error!("unable to edit user: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;
    }

    Ok(message_response::ok("success"))
}
