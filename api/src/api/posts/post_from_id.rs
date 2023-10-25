use crate::api::response::{message_response, Message, Response};
use crate::db::models::{Permission, Post};
use crate::db::{database::DatabaseParam, models::Id};
use crate::permission_verification;
use salvo::session::SessionDepotExt;
use salvo::{oapi::extract::PathParam, prelude::ToSchema, Depot};
use serde::Serialize;

#[derive(Serialize, ToSchema)]
struct RouteResponse {
    ok: bool,
    data: Post,
}

#[salvo::endpoint(status_codes(200, 400, 500))]
pub async fn route(
    category_id: PathParam<Id>,
    post_id: PathParam<Id>,
    depot: &mut Depot,
) -> Result<Response<RouteResponse>, Response<Message>> {
    let user_id = depot
        .session()
        .and_then(|session| session.get::<Id>("user_id"));
    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to get database from depot: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;

    let db = db.read().await;
    let permission = if let Some(user_id) = user_id {
        db.user_from_id(&user_id)
            .await
            .map_err(|err| log::error!("unable to get user from id: {err:?}"))
            .map_err(|()| message_response::internal_server_error("internal server error"))?
            .map_or(Permission::default(), |user| user.permission)
    } else {
        Permission::default()
    };

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
        .post_from_id(&post_id)
        .await
        .map_err(|err| {
            log::error!("unable to get post from category with id {category_id}: {err:?}");
        })
        .map_err(|()| message_response::internal_server_error("internal server error"))?;

    if data.as_ref().is_some_and(|v| v.category_id != category.id) {
        return Err(message_response::bad_request("invalid category or post id"));
    }

    match data {
        Some(data) => Ok(Response::with_ok(RouteResponse { data, ok: true })),
        None => Err(message_response::bad_request("invalid post id")),
    }
}
