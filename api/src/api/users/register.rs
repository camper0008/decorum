use salvo::{
    http::errors::StatusResult,
    oapi::extract::JsonBody,
    prelude::{Extractible, StatusCode, StatusError, ToSchema},
    Depot,
};
use serde::Deserialize;

use crate::db::{
    database::{CreateUser, DatabaseParam},
    models::Permission,
};

#[derive(Deserialize, Extractible, ToSchema)]
struct RouteRequest {
    username: String,
    password: String,
}

#[salvo::endpoint(status_codes(201, 400, 500))]
pub async fn route(request: JsonBody<RouteRequest>, depot: &mut Depot) -> StatusResult<StatusCode> {
    let JsonBody(RouteRequest { username, password }) = request;

    if username.trim().is_empty() || password.trim().is_empty() {
        return Err(StatusError::bad_request().brief("invalid username or password"));
    }

    let db = depot
        .obtain::<DatabaseParam>()
        .map_err(|err| log::error!("unable to obtain database from depot: {err:?}"))
        .map_err(|()| StatusError::internal_server_error().brief("internal server error"))?;

    {
        let db = db.read().await;
        let user = db.user_from_username(&username).await.map_err(|err| {
            log::error!("unable to read username from db: {err:?}");
            StatusError::internal_server_error().brief("internal server error")
        })?;

        if user.is_some() {
            return Err(StatusError::bad_request().brief("user already exists"));
        }
    }
    log::info!("b={password}");
    let password = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|err| log::error!("unable to hash pw: {err:?}"))
        .map_err(|()| StatusError::internal_server_error().brief("internal server error"))?;
    log::info!("a={password}");
    {
        let mut db = db.write().await;
        let nickname = username.clone();
        db.create_user(CreateUser {
            username,
            nickname,
            password,
            permission: Permission::Unverified,
            avatar_id: None,
        })
        .await
        .map_err(|err| log::error!("unable to save user in db: {err:?}"))
        .map_err(|()| StatusError::internal_server_error().brief("error creating post"))?;
    }

    Ok(StatusCode::CREATED)
}
