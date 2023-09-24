use salvo::session::SessionDepotExt;
use salvo::{Depot, Request};
use tokio::sync::RwLockReadGuard;

use crate::permission_verification;
use crate::{
    api::response::{message_response, CreatedResponseResult},
    db::database::CreateAttachment,
};
use crate::{
    api::response::{Message, Response},
    db::{
        database::{Database, DatabaseParam},
        models::Id,
    },
};

async fn verify_valid_user_permission<'a, Db: Database + Sync + Send + ?Sized>(
    db: &RwLockReadGuard<'_, Db>,
    user_id: &Id,
) -> Result<(), Response<Message>> {
    let user = db
        .user_from_id(user_id)
        .await
        .map_err(|_| message_response::internal_server_error("internal server error"))?
        .ok_or_else(|| message_response::unauthorized("invalid session"))?;

    let attachment_permission = permission_verification::permission_for_attachment_upload();

    if !permission_verification::is_allowed(&user.permission, &attachment_permission) {
        let err = format!(
            "you must be {} or above to upload attachments, you are {}",
            attachment_permission, user.permission
        );
        return Err(message_response::unauthorized(err));
    }

    Ok(())
}

#[salvo::endpoint(status_codes(201, 400, 403, 500))]
pub async fn route(depot: &mut Depot, request: &mut Request) -> CreatedResponseResult {
    let creator_id = depot
        .session()
        .and_then(|session| session.get::<Id>("user_id"))
        .ok_or_else(|| message_response::unauthorized("invalid session"))?;
    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to get database from depot: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;

    let file = request
        .first_file()
        .await
        .ok_or_else(|| message_response::bad_request("missing file"))?;

    {
        let db = db.read().await;
        verify_valid_user_permission(&db, &creator_id).await?;
    }
    let id = {
        let mut db = db.write().await;
        db.create_attachment(CreateAttachment {
            file_name: file.name().unwrap_or("file"),
            temp_path: file.path(),
            creator_id,
        })
        .await
        .map_err(|err| log::error!("unable to save attachment in database: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?
    };

    Ok(message_response::created_with_id("created", id))
}
