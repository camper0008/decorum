use salvo::{
    oapi::extract::JsonBody,
    prelude::{Extractible, ToSchema},
    session::SessionDepotExt,
    Depot,
};
use serde::Deserialize;
use tokio::sync::RwLockReadGuard;

use crate::{api::response::MessageResponseResult, db::database::EditReply};
use crate::{api::response::Response, permission_verification};
use crate::{
    api::response::{message_response, Message},
    db::{
        database::{Database, DatabaseParam},
        models::Id,
    },
};

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    id: String,
}

async fn verify_valid_user_permission<'a, Db: Database + Sync + Send + ?Sized>(
    db: &RwLockReadGuard<'_, Db>,
    user_id: &Id,
    reply_creator_id: &Id,
    post_id: &Id,
) -> Result<(), Response<Message>> {
    let user = db
        .user_from_id(user_id)
        .await
        .map_err(|err| log::error!("unable to get user with id '{}': {err:?}", user_id))
        .map_err(|()| message_response::internal_server_error("internal server error"))?
        .ok_or_else(|| message_response::unauthorized("invalid session"))?;

    let post = db
        .post_from_id(post_id)
        .await
        .map_err(|err| log::error!("unable to get post with id '{}': {err:?}", post_id))
        .map_err(|()| message_response::internal_server_error("internal server error"))?
        .ok_or_else(|| message_response::bad_request("invalid post id"))?;

    let category = db
        .category_from_id(&post.category_id)
        .await
        .map_err(|err| {
            log::error!(
                "unable to get category with id '{}': {err:?}",
                post.category_id,
            );
        })
        .map_err(|()| message_response::internal_server_error("internal server error"))?
        .ok_or_else(|| message_response::bad_request("invalid category id"))?;

    let remove_permission = permission_verification::permission_for_important_actions();

    if reply_creator_id != user_id
        && !permission_verification::is_allowed(&user.permission, &remove_permission)
    {
        return Err(message_response::unauthorized("invalid reply id"));
    }

    if !permission_verification::is_allowed(&user.permission, &category.minimum_write_permission) {
        let err = format!(
            "you must be {} or above to create replies in category {}, you are {}",
            category.minimum_write_permission, category.title, user.permission
        );
        return Err(message_response::unauthorized(err));
    }

    Ok(())
}

#[salvo::endpoint(status_codes(200, 400, 403, 500))]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> MessageResponseResult {
    let JsonBody(RouteRequest { id }) = request;

    let id = Id::try_from(id).map_err(|_| message_response::bad_request("invalid reply id"))?;

    let user_id = depot
        .session()
        .and_then(|session| session.get::<Id>("user_id"))
        .ok_or_else(|| message_response::unauthorized("invalid session"))?;
    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to get database from depot: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;

    let reply = {
        let db = db.read().await;
        let reply = db
            .reply_from_id(&id)
            .await
            .map_err(|err| log::error!("unable to get reply from database: {err:?}"))
            .map_err(|()| message_response::internal_server_error("internal server error"))?
            .ok_or_else(|| message_response::bad_request("invalid reply id"))?;
        verify_valid_user_permission(&db, &user_id, &reply.creator_id, &reply.post_id).await?;
        reply
    };
    {
        let mut db = db.write().await;
        db.edit_reply(EditReply {
            id: reply.id,
            content: reply.content,
            deleted: true,
        })
        .await
        .map_err(|err| log::error!("unable to save post in database: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;
    }

    Ok(message_response::created("created"))
}
