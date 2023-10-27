use salvo::{session::SessionDepotExt, Depot};

use crate::api::response::{message_response, MessageResponseResult};

#[salvo::endpoint(status_codes(200, 400, 500))]
pub async fn route(depot: &mut Depot) -> MessageResponseResult {
    match depot.session_mut() {
        Some(session) => session.remove("user_id"),
        None => return Err(message_response::bad_request("invalid session")),
    };
    Ok(message_response::ok("success"))
}
