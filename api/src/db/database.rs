use std::sync::Arc;

use salvo::async_trait;
use tokio::sync::RwLock;

use super::models::User;

pub type DatabaseError = snafu::Whatever;

pub type DatabaseParam = Arc<RwLock<dyn Database + Send + Sync>>;

#[async_trait]
pub trait Database {
    async fn test(&self) -> Result<User, DatabaseError>;

    // async fn create_user(
    //     &mut self,
    //     username: String,
    //     nickname: String,
    //     password: String,
    //     avatar: Option<Attachment>,
    // ) -> Result<User, DatabaseError>;
    // async fn delete_user(
    //     &mut self,
    //     username: String,
    //     nickname: String,
    //     password: String,
    //     avatar: Option<Attachment>,
    // ) -> Result<User, DatabaseError>;
}
