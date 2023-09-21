mod api;
mod db;
mod iso_date_strings;
mod permission_verification;

use std::sync::Arc;

use db::{database::DatabaseParam, sqlite::SqliteDb};
use eyre::Context;
use salvo::http::cookie::time::Duration;
use salvo::rate_limiter::{BasicQuota, FixedGuard, MokaStore, RateLimiter, RemoteIpIssuer};
use salvo::{prelude::*, session::CookieStore};
use tokio::sync::RwLock;

fn openapi_route(router: Router) -> Router {
    let doc = OpenApi::new("Decorum API", env!("CARGO_PKG_VERSION")).merge_router(&router);
    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"))
}

// TODO: everything else api

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

    let limiter = RateLimiter::new(
        FixedGuard::new(),
        MokaStore::new(),
        RemoteIpIssuer,
        BasicQuota::new(1, Duration::seconds_f64(15.0)),
    );

    let router = Router::new();
    let router = router
        .hoop(session_handler)
        .hoop(affix::inject::<DatabaseParam>(database))
        .hoop(limiter)
        .push(Router::with_path("/users/register").post(api::users::register_route))
        .push(Router::with_path("/users/login").post(api::users::login_route))
        .push(Router::with_path("/posts/create_post").post(api::posts::create_post_route))
        .push(Router::with_path("/posts/create_category").post(api::posts::create_category_route));

    let router = openapi_route(router);

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
    Ok(())
}
