use crate::{
    api::response::{message_response, Message, Response},
    db::models::{Permission, Post},
};
use crate::{
    db::{database::DatabaseParam, models::Id},
    permission_verification,
};
use salvo::{oapi::extract::PathParam, prelude::ToSchema, session::SessionDepotExt, Depot};
use serde::Serialize;

#[derive(Serialize, ToSchema)]
struct RouteResponse {
    ok: bool,
    data: Vec<Post>,
}

#[salvo::endpoint(status_codes(200, 400, 500))]
pub async fn route(
    category_id: PathParam<Id>,
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

    let db = db.read().await;
    let permission = db
        .user_from_id(&user_id)
        .await
        .map_err(|err| log::error!("unable to get user from id: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?
        .map_or(Permission::Unverified, |user| user.permission);

    let category = db
        .category_from_id(&category_id)
        .await
        .map_err(|err| log::error!("unable to get all categories: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?
        .ok_or_else(|| message_response::bad_request("invalid category id"))?;

    if !permission_verification::is_allowed(&permission, &category.minimum_read_permission) {
        let err = format!(
            "you must be {} or above to read posts in category {}, you are {}",
            category.minimum_read_permission, category.title, permission
        );
        return Err(message_response::unauthorized(err));
    };

    let data = db
        .posts_from_category(&category_id)
        .await
        .map_err(|err| {
            log::error!("unable to get posts from category with id {category_id}: {err:?}");
        })
        .map_err(|()| message_response::internal_server_error("internal server error"))?;

    Ok(Response::with_ok(RouteResponse { data, ok: true }))
}
