mod api;
mod db;
mod permission_utils;

use std::sync::Arc;

use db::{database::DatabaseParam, sqlite::SqliteDb};
use salvo::prelude::*;
use tokio::sync::RwLock;

fn openapi_route(router: Router) -> Router {
    let doc = OpenApi::new("Decorum API", env!("CARGO_PKG_VERSION")).merge_router(&router);
    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let database = Arc::new(RwLock::new(SqliteDb {}));

    let router = Router::new();
    let router = router.push(Router::with_path("/health").get(api::health::route));
    let router = router
        .hoop(affix::inject::<DatabaseParam>(database))
        .push(Router::with_path("/posts/create_post").post(api::posts::create_post_route));

    let router = openapi_route(router);

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
