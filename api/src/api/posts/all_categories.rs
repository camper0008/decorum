use crate::{
    api::response::{message_response, Message, Response},
    db::models::{Category, Permission},
};
use crate::{
    db::{database::DatabaseParam, models::Id},
    permission_verification,
};
use salvo::{prelude::ToSchema, session::SessionDepotExt, Depot};
use serde::Serialize;

#[derive(Serialize, ToSchema)]
struct RouteResponse {
    ok: bool,
    data: Vec<Category>,
}

#[salvo::endpoint(status_codes(200, 400, 500))]
pub async fn route(depot: &mut Depot) -> Result<Response<RouteResponse>, Response<Message>> {
    let user_id = depot
        .session()
        .and_then(|session| session.get::<Id>("user_id"))
        .ok_or_else(|| message_response::unauthorized("invalid session"))?;
    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to get database from depot: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;

    let db = db.read().await;
    let permission = db
        .user_from_id(&user_id)
        .await
        .map_err(|err| log::error!("unable to get user from id: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?
        .map_or(Permission::Unverified, |user| user.permission);
    let categories = db
        .all_categories()
        .await
        .map_err(|err| log::error!("unable to get all categories: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?
        .into_iter()
        .filter(|category| {
            permission_verification::is_allowed(&permission, &category.minimum_read_permission)
        })
        .collect();

    Ok(Response::with_ok(RouteResponse {
        data: categories,
        ok: true,
    }))
}
