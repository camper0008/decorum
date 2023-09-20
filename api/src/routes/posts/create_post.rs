use salvo::{oapi::extract::QueryParam, Depot};

use crate::db::database::DatabaseParam;

#[salvo::endpoint]
pub async fn route(name: QueryParam<String, false>, depot: &mut Depot) -> String {
    let db = depot.obtain::<DatabaseParam>();
    println!("reached");
    match db {
        Ok(db) => {
            println!("db ok");
            let db = db.read().await;
            println!("db unlocked");
            let _ = db.test().await;
        }
        Err(err) => {
            dbg!(err);
        }
    }
    format!("Hello, {}!", name.as_deref().unwrap_or("World"))
}
