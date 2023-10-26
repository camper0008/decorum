use crate::db::{database::DatabaseParam, models::Id};
use crate::{
    api::response::{message_response, Message, Response},
    db::models::{Name, Permission},
};
use salvo::session::SessionDepotExt;
use salvo::{prelude::ToSchema, Depot};
use serde::Serialize;

#[derive(Serialize, ToSchema)]
struct ResponseUser {
    id: Id,
    username: Name,
    nickname: Option<Name>,
    permission: Permission,
    avatar_id: Option<Id>,
    date_created: String,
}

#[derive(Serialize, ToSchema)]
struct RouteResponse {
    ok: bool,
    data: ResponseUser,
}

#[salvo::endpoint(status_codes(200, 400, 500))]
pub async fn route(
    depot: &mut Depot,
) -> Result<Response<RouteResponse>, Response<Message>> {
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

    let data = ResponseUser {
        id: user.id,
        username: user.username,
        nickname: user.nickname,
        permission: user.permission,
        avatar_id: user.avatar_id,
        date_created: user.date_created,
    };

    Ok(Response::with_ok(RouteResponse { data, ok: true }))
}
