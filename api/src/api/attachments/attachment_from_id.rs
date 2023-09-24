use crate::api::response::{message_response, Message, Response};
use crate::db::{database::DatabaseParam, models::Id};
use salvo::{oapi::extract::PathParam, Depot};

#[salvo::endpoint(status_codes(200, 400, 500))]
pub async fn route(
    attachment_id: PathParam<Id>,
    depot: &mut Depot,
    request: &mut salvo::Request,
    response: &mut salvo::Response,
) -> Result<(), Response<Message>> {
    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to get database from depot: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?;
    let db = db.read().await;
    let attachment = db
        .attachment_from_id(&attachment_id)
        .await
        .map_err(|err| log::error!("unable to get attachment from id: {err:?}"))
        .map_err(|()| message_response::internal_server_error("internal server error"))?
        .ok_or_else(|| message_response::bad_request("invalid attachment id"))?;

    response.send_file(attachment.path, request.headers()).await;

    Ok(())
}
