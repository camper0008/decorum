use salvo::{prelude::ToSchema, writing::Json};
use serde::Serialize;

#[derive(Serialize, ToSchema)]
struct RouteResponse<'a> {
    message: &'a str,
}

#[salvo::endpoint]
pub async fn route() -> Json<RouteResponse<'static>> {
    Json(RouteResponse { message: "OK" })
}
