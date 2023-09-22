use salvo::{
    oapi::extract::JsonBody,
    prelude::{Extractible, ToSchema},
    session::SessionDepotExt,
    Depot,
};
use serde::Deserialize;
use tokio::sync::RwLockReadGuard;

use crate::{api::response::MessageResponseResult, db::database::CreateReply};
use crate::{api::response::Response, permission_verification};
use crate::{
    api::response::{message_response, Message},
    db::{
        database::{Database, DatabaseParam},
        models::{Content, Id},
    },
};

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    post_id: String,
    content: String,
}

async fn verify_valid_user_permission<'a, Db: Database + Sync + Send + ?Sized>(
    db: &RwLockReadGuard<'_, Db>,
    user_id: &Id,
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

    if !permission_verification::is_allowed(&user.permission, &category.minimum_write_permission) {
        let err = format!(
            "you must be {} or above to create replies in category {}, you are {}",
            category.minimum_write_permission, category.title, user.permission
        );
        return Err(message_response::unauthorized(err));
    }

    Ok(())
}

#[salvo::endpoint(status_codes(201, 400, 403, 500))]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> MessageResponseResult {
    let JsonBody(RouteRequest { post_id, content }) = request;

    let post_id =
        Id::try_from(post_id).map_err(|_| message_response::bad_request("invalid post id"))?;
    let content =
        Content::try_from(content).map_err(|_| message_response::bad_request("invalid content"))?;

    let creator_id = depot
        .session()
        .and_then(|session| session.get::<Id>("user_id"))
        .ok_or_else(|| message_response::unauthorized("invalid session"))?;
    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to get database from depot: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;

    {
        let db = db.read().await;
        verify_valid_user_permission(&db, &creator_id, &post_id).await?;
    }
    {
        let mut db = db.write().await;
        db.create_reply(CreateReply {
            creator_id,
            post_id,
            content,
        })
        .await
        .map_err(|err| log::error!("unable to save post in database: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;
    }

    Ok(message_response::created("created"))
}
