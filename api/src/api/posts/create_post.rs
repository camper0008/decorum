use salvo::{
    oapi::extract::JsonBody,
    prelude::{Extractible, ToSchema},
    session::SessionDepotExt,
    Depot,
};
use serde::Deserialize;
use tokio::sync::RwLockReadGuard;

use crate::api::response::{message_response, CreatedResponseResult};
use crate::{
    api::response::{Message, Response},
    db::{
        database::{CreatePost, Database, DatabaseParam},
        models::{Content, Id},
    },
};
use crate::{db::models::Title, permission_verification};

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    category_id: String,
    title: String,
    content: String,
}

async fn verify_valid_user_permission<'a, Db: Database + Sync + Send + ?Sized>(
    db: &RwLockReadGuard<'_, Db>,
    user_id: &Id,
    category_id: &Id,
) -> Result<(), Response<Message>> {
    let user = db
        .user_from_id(user_id)
        .await
        .map_err(|_| message_response::internal_server_error("internal server error"))?
        .ok_or_else(|| message_response::unauthorized("invalid session"))?;

    let category = db
        .category_from_id(category_id)
        .await
        .map_err(|_| message_response::internal_server_error("internal server error"))?
        .ok_or_else(|| message_response::bad_request("invalid category id"))?;

    if !permission_verification::is_allowed(&user.permission, &category.minimum_write_permission) {
        let err = format!(
            "you must be {} or above to create posts in category {}, you are {}",
            category.minimum_write_permission, category.title, user.permission
        );
        return Err(message_response::unauthorized(err));
    }

    Ok(())
}

#[salvo::endpoint(status_codes(201, 400, 403, 500))]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> CreatedResponseResult {
    let JsonBody(RouteRequest {
        category_id,
        title,
        content,
    }) = request;

    let category_id = Id::try_from(category_id)
        .map_err(|_| message_response::bad_request("invalid category id"))?;
    let title =
        Title::try_from(title).map_err(|_| message_response::bad_request("invalid title"))?;
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
        verify_valid_user_permission(&db, &creator_id, &category_id).await?;
    }
    let id = {
        let mut db = db.write().await;
        db.create_post(CreatePost {
            category_id,
            title,
            content,
            creator_id,
        })
        .await
        .map_err(|err| log::error!("unable to save post in database: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?
    };

    Ok(message_response::created_with_id("created", id))
}
