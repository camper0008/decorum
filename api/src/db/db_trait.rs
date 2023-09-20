use salvo::async_trait;

use super::models::{Attachment, User};

pub type DatabaseError = snafu::Whatever;

#[async_trait]
pub trait Database {
    async fn create_user(
        username: String,
        nickname: String,
        password: String,
        avatar: Option<Attachment>,
    ) -> Result<User, DatabaseError>;
    async fn delete_user(
        username: String,
        nickname: String,
        password: String,
        avatar: Option<Attachment>,
    ) -> Result<User, DatabaseError>;
}
