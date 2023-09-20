use snafu::whatever;

use super::{
    database::{Database, DatabaseError},
    models::User,
};

pub struct SqliteDb {}

#[salvo::async_trait]
impl Database for SqliteDb {
    async fn test(&self) -> Result<User, DatabaseError> {
        println!("test");
        whatever!("unimplemented");
    }
}
