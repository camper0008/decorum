use salvo::{
    http::errors::StatusResult,
    oapi::extract::JsonBody,
    prelude::{Extractible, StatusError, ToSchema},
    Depot,
};
use serde::Deserialize;

use crate::db::database::{CreateUser, DatabaseParam};

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    username: String,
    password: String,
}

#[salvo::endpoint]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> StatusResult<()> {
    let JsonBody(RouteRequest { username, password }) = request;

    if username.trim().is_empty() || password.trim().is_empty() {
        return Err(StatusError::bad_request().brief("invalid username or password"));
    }

    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|_| StatusError::internal_server_error().brief("internal server error"))?;

    {
        let db = db.read().await;
        let user = db
            .user_from_username(&username)
            .await
            .map_err(|_| StatusError::internal_server_error().brief("internal server error"))?;

        if user.is_some() {
            return Err(StatusError::bad_request().brief("user already exists"));
        }
    }
    let password = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|_| StatusError::internal_server_error().brief("internal server error"))?;
    {
        let mut db = db.write().await;
        let nickname = username.clone();
        db.create_user(CreateUser {
            username,
            nickname,
            password,
            avatar: None,
        })
        .await
        .map_err(|_| StatusError::internal_server_error().brief("error creating post"))?;
    }

    Ok(())
}
