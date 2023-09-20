use std::sync::Arc;

use salvo::async_trait;
use tokio::sync::RwLock;

use super::models::{Attachment, Category, Id, Post, User};

pub type DatabaseError = eyre::Report;

pub type DatabaseParam = Arc<RwLock<dyn Database + Send + Sync>>;

#[async_trait]
pub trait Database {
    async fn create_user(
        &mut self,
        username: String,
        nickname: String,
        password: String,
        avatar: Option<Attachment>,
    ) -> Result<User, DatabaseError>;
    async fn delete_user(
        &mut self,
        username: String,
        nickname: String,
        password: String,
        avatar: Option<Attachment>,
    ) -> Result<User, DatabaseError>;
    async fn create_post(
        &mut self,
        category: Id,
        title: String,
        content: String,
        creator_id: Id,
    ) -> Result<Post, DatabaseError>;
    async fn user_from_id(&self, id: &Id) -> Result<User, DatabaseError>;
    async fn category_from_id(&self, id: &Id) -> Result<Category, DatabaseError>;
}
