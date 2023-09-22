use salvo::{
    oapi::extract::JsonBody,
    prelude::{Extractible, ToSchema},
    session::SessionDepotExt,
    Depot,
};
use serde::Deserialize;
use tokio::sync::RwLockReadGuard;

use crate::db::{
    database::{CreatePost, Database, DatabaseParam},
    models::{Content, Id},
};
use crate::permission_verification;
use crate::{
    api::response::{MessageResponse, ResponseResult},
    db::models::Name,
};

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
) -> Result<(), MessageResponse> {
    let user = db
        .user_from_id(&user_id)
        .await
        .map_err(|_| MessageResponse::internal_server_error("internal server error"))?
        .ok_or_else(|| MessageResponse::unauthorized("invalid session"))?;

    let category = db
        .category_from_id(&category_id)
        .await
        .map_err(|_| MessageResponse::internal_server_error("internal server error"))?
        .ok_or_else(|| MessageResponse::bad_request("invalid category id"))?;

    if !permission_verification::is_allowed(&user.permission, &category.minimum_write_permission) {
        let err = format!(
            "you must be {} or above to create posts in category {}, you are {}",
            category.minimum_write_permission,
            category.title.to_string(),
            user.permission
        );
        return Err(MessageResponse::unauthorized(err));
    }

    Ok(())
}

#[salvo::endpoint(status_codes(201, 400, 403, 500))]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> ResponseResult {
    let JsonBody(RouteRequest {
        category_id,
        title,
        content,
    }) = request;

    let category_id = Id::try_from(category_id)
        .map_err(|_| MessageResponse::bad_request("invalid category id"))?;
    let title = Name::try_from(title).map_err(|_| MessageResponse::bad_request("invalid title"))?;
    let content =
        Content::try_from(content).map_err(|_| MessageResponse::bad_request("invalid content"))?;

    let creator_id = depot
        .session()
        .map(|session| session.get::<Id>("user_id"))
        .flatten()
        .ok_or_else(|| MessageResponse::unauthorized("invalid session"))?;
    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to get database from depot: {err:?}"))
        .map_err(|()| MessageResponse::internal_server_error("internal server error"))?;

    {
        let db = db.read().await;
        verify_valid_user_permission(&db, &creator_id, &category_id).await?;
    }
    {
        let mut db = db.write().await;
        db.create_post(CreatePost {
            category_id,
            title,
            content,
            creator_id,
        })
        .await
        .map_err(|err| log::error!("unable to save post in database: {err:?}"))
        .map_err(|()| MessageResponse::internal_server_error("internal server error"))?;
    }

    Ok(MessageResponse::created("created"))
}
