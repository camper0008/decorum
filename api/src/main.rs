#![allow(clippy::module_name_repetitions)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::manual_unwrap_or)]
#![warn(clippy::map_unwrap_or)]

mod api;
mod db;
mod from_unchecked;
mod iso_date_strings;
mod password;
mod permission_verification;

use std::sync::Arc;

use db::{database::DatabaseParam, sqlite::SqliteDb};
use eyre::Context;
use salvo::rate_limiter::{BasicQuota, FixedGuard, MokaStore, RateLimiter, RemoteIpIssuer};
use salvo::{prelude::*, session::CookieStore};
use tokio::sync::RwLock;

fn openapi_route(router: Router) -> Router {
    let doc = OpenApi::new("Decorum API", env!("CARGO_PKG_VERSION")).merge_router(&router);
    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"))
}

fn read_routes() -> Router {
    let limiter = RateLimiter::new(
        FixedGuard::new(),
        MokaStore::new(),
        RemoteIpIssuer,
        BasicQuota::per_second(30),
    );
    Router::with_hoop(limiter)
        .push(Router::with_path("/posts/all_categories").get(api::posts::all_categories_route))
        .push(
            Router::with_path("/posts/posts_from_category/<category_id>")
                .get(api::posts::posts_from_category_route),
        )
        .push(
            Router::with_path("/posts/post_from_id/<category_id>/<post_id>")
                .get(api::posts::post_from_id_route),
        )
        .push(
            Router::with_path("/posts/replies_from_post/<post_id>")
                .get(api::posts::replies_from_post_route),
        )
        .push(
            Router::with_path("/users/user_from_id/<user_id>").get(api::users::user_from_id_route),
        )
        .push(
            Router::with_path("/users/user_from_session").get(api::users::user_from_session_route),
        )
        .push(
            Router::with_path("/attachments/attachment_from_id/<attachment_id>")
                .get(api::attachments::attachment_from_id_route),
        )
}

fn write_routes() -> Router {
    let limiter = RateLimiter::new(
        FixedGuard::new(),
        MokaStore::new(),
        RemoteIpIssuer,
        BasicQuota::per_minute(10),
    );
    Router::with_hoop(limiter)
        .push(Router::with_path("/users/register").post(api::users::register_route))
        .push(Router::with_path("/users/login").post(api::users::login_route))
        .push(Router::with_path("/users/edit_user").post(api::users::edit_user_route))
        .push(
            Router::with_path("/users/edit_user_permission")
                .post(api::users::edit_user_permission_route),
        )
        .push(Router::with_path("/posts/create_post").post(api::posts::create_post_route))
        .push(Router::with_path("/posts/create_category").post(api::posts::create_category_route))
        .push(Router::with_path("/posts/create_reply").post(api::posts::create_reply_route))
        .push(Router::with_path("/posts/lock_post").post(api::posts::lock_post_route))
        .push(Router::with_path("/posts/edit_post").post(api::posts::edit_post_route))
        .push(Router::with_path("/posts/edit_category").post(api::posts::edit_category_route))
        .push(Router::with_path("/posts/edit_reply").post(api::posts::edit_reply_route))
        .push(Router::with_path("/posts/remove_post").post(api::posts::remove_post_route))
        .push(Router::with_path("/posts/remove_category").post(api::posts::remove_category_route))
        .push(Router::with_path("/posts/remove_reply").post(api::posts::remove_reply_route))
        .push(Router::with_path("/posts/edit_post_lock_status").post(api::posts::lock_post_route))
        .push(
            Router::with_path("/attachments/create_attachment")
                .post(api::attachments::create_attachment_route),
        )
}

/// TODO: 'wipe' option?
/// TODO: attachment get
/// TODO: attachment upload

#[tokio::main]
async fn main() -> eyre::Result<()> {
    if dotenv::dotenv().is_err() {
        println!("unable to find .env file");
    };

    tracing_subscriber::fmt().init();

    let session_handler_token = std::env::var("SESSION_HANDLER_TOKEN")
        .with_context(|| "env variable `SESSION_HANDLER_TOKEN` should be set")?;

    let session_handler =
        SessionHandler::builder(CookieStore::new(), session_handler_token.as_bytes())
            .build()
            .with_context(|| "env variable `SESSION_HANDLER_TOKEN` invalid")?;

    let database_url = std::env::var("DATABASE_URL")
        .with_context(|| "env variable `DATABASE_URL` should be set")?;

    let database = SqliteDb::new(database_url).await?;
    let database = Arc::new(RwLock::new(database));

    let router = Router::new();

    let router = router.push(
        Router::new()
            .hoop(session_handler)
            .hoop(affix::inject::<DatabaseParam>(database))
            .push(write_routes())
            .push(read_routes()),
    );

    let router = openapi_route(router);

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
    Ok(())
}
