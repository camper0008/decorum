use salvo::{
    oapi::extract::JsonBody,
    prelude::{Extractible, ToSchema},
    session::SessionDepotExt,
    Depot,
};
use serde::Deserialize;
use tokio::sync::RwLockReadGuard;

use crate::api::response::{MessageResponse, ResponseResult};
use crate::db::{
    database::{CreatePost, Database, DatabaseParam},
    models::Id,
};
use crate::permission_utils;

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    category_id: String,
    title: String,
    content: String,
}

async fn verify_valid_user_and_category<'a, Db: Database + Sync + Send + ?Sized>(
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

    if !permission_utils::is_allowed(&user.permission, &category.minimum_write_permission) {
        let err = format!(
            "you must be {} or above to create posts in category {}, you are {}",
            category.minimum_write_permission, category.name.0, user.permission
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

    let category_id = Id::from(category_id);

    if title.trim().is_empty() || content.trim().is_empty() {
        return Err(MessageResponse::bad_request(
            "title or content field is empty",
        ));
    }

    let user_id = depot
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
        verify_valid_user_and_category(&db, &user_id, &category_id).await?;
    }
    {
        let mut db = db.write().await;
        db.create_post(CreatePost {
            category_id,
            title,
            content,
            creator_id: user_id,
        })
        .await
        .map_err(|err| log::error!("unable to save post in database: {err:?}"))
        .map_err(|()| MessageResponse::internal_server_error("internal server error"))?;
    }

    Ok(MessageResponse::ok("created"))
}
