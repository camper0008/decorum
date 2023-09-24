use salvo::{
    oapi::extract::JsonBody,
    prelude::{Extractible, ToSchema},
    session::SessionDepotExt,
    Depot,
};
use serde::Deserialize;
use tokio::sync::RwLockReadGuard;

use crate::{api::response::message_response, db::models::Permission};
use crate::{api::response::Response, permission_verification};
use crate::{
    api::response::{CreatedResponseResult, Message},
    db::{
        database::{CreateCategory, Database, DatabaseParam},
        models::Id,
    },
};

#[derive(Deserialize, Extractible, ToSchema)]
struct MinimumPermissionRequest {
    read: Permission,
    write: Permission,
}

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    title: String,
    minimum_permissions: MinimumPermissionRequest,
}

async fn verify_valid_user_permission<'a, Db: Database + Sync + Send + ?Sized>(
    db: &RwLockReadGuard<'_, Db>,
    user_id: &Id,
    minimum_read_permission: &Permission,
    minimum_write_permission: &Permission,
) -> Result<(), Response<Message>> {
    let user = db
        .user_from_id(user_id)
        .await
        .map_err(|_| message_response::internal_server_error("internal server error"))?
        .ok_or_else(|| message_response::unauthorized("invalid session"))?;

    let category_permission = permission_verification::permission_for_important_actions();

    if !permission_verification::is_allowed(&user.permission, &category_permission) {
        let err = format!(
            "you must be {} or above to create categories, you are {}",
            category_permission, user.permission,
        );
        return Err(message_response::unauthorized(err));
    }

    if !permission_verification::is_allowed(&user.permission, minimum_write_permission) {
        let err = format!(
            "you must be {} or above to create categories with write permission {}, you are {}",
            minimum_write_permission, minimum_write_permission, user.permission
        );
        return Err(message_response::unauthorized(err));
    }
    if !permission_verification::is_allowed(&user.permission, minimum_read_permission) {
        let err = format!(
            "you must be {} or above to create categories with read permission {}, you are {}",
            minimum_read_permission, minimum_read_permission, user.permission
        );
        return Err(message_response::unauthorized(err));
    }

    Ok(())
}

#[salvo::endpoint(status_codes(201, 400, 403, 500))]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> CreatedResponseResult {
    let JsonBody(RouteRequest {
        title,
        minimum_permissions:
            MinimumPermissionRequest {
                read: read_permission,
                write: write_permission,
            },
    }) = request;

    let title = title
        .try_into()
        .map_err(|_| message_response::bad_request("invalid title"))?;
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
        verify_valid_user_permission(&db, &creator_id, &read_permission, &write_permission).await?;
    }
    let id = {
        let mut db = db.write().await;
        db.create_category(CreateCategory {
            title,
            minimum_read_permission: read_permission,
            minimum_write_permission: write_permission,
        })
        .await
        .map_err(|err| log::error!("unable to save post in database: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?
    };

    Ok(message_response::created_with_id("created", id))
}
