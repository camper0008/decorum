use salvo::{
    http::errors::StatusResult,
    oapi::extract::JsonBody,
    prelude::{Extractible, StatusError, ToSchema},
    session::SessionDepotExt,
    Depot,
};
use serde::Deserialize;

use crate::db::{database::DatabaseParam, models::Id};
use crate::permission_utils;

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    category: String,
    title: String,
    content: String,
}

#[salvo::endpoint]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> StatusResult<()> {
    let JsonBody(RouteRequest {
        category,
        title,
        content,
    }) = request;

    let category = Id::from(category);

    if title.trim().is_empty() || content.trim().is_empty() {
        return Err(StatusError::bad_request().brief("title or content field is empty"));
    }

    let user_id = depot
        .session()
        .map(|session| session.get::<Id>("user_id"))
        .flatten()
        .ok_or_else(|| StatusError::unauthorized().brief("invalid session"))?;
    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|_| StatusError::internal_server_error().brief("internal server error"))?;

    {
        let db = db.read().await;
        let user = db
            .user_from_id(&user_id)
            .await
            .map_err(|_| StatusError::unauthorized().brief("invalid session"))?;

        let category = db
            .category_from_id(&category)
            .await
            .map_err(|_| StatusError::bad_request().brief("invalid category id"))?;

        if !permission_utils::is_allowed(&user.permission, &category.minimum_permission) {
            return Err(StatusError::unauthorized().brief(format!(
                "you must be {} or above to create posts in category {}, you are {}",
                category.minimum_permission, category.name, user.permission
            )));
        }
    }
    {
        let mut db = db.write().await;
        db.create_post(category, title, content, user_id)
            .await
            .map_err(|_| StatusError::internal_server_error().brief("error creating post"))?;
    }

    Ok(())
}
