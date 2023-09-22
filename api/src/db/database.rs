use std::sync::Arc;

use salvo::async_trait;
use tokio::sync::RwLock;

use crate::password::HashedPassword;

use super::models::{Category, Content, Id, Name, Permission, Post, Reply, User};

pub type DatabaseError = eyre::Report;

pub type DatabaseParam = Arc<RwLock<dyn Database + Send + Sync>>;

pub struct CreateUser {
    pub username: Name,
    pub nickname: Option<Name>,
    pub password: HashedPassword,
    pub permission: Permission,
    pub avatar_id: Option<Id>,
}

pub struct CreatePost {
    pub category_id: Id,
    pub title: Name,
    pub content: Content,
    pub creator_id: Id,
}

pub struct CreateCategory {
    pub title: Name,
    pub minimum_write_permission: Permission,
    pub minimum_read_permission: Permission,
}
pub struct CreateReply {
    pub creator_id: Id,
    pub post_id: Id,
    pub content: Content,
}

#[async_trait]
pub trait Database {
    async fn create_user(&mut self, data: CreateUser) -> Result<User, DatabaseError>;
    async fn delete_user_with_id(&mut self, id: &Id) -> Result<Option<User>, DatabaseError>;
    async fn user_from_id(&self, id: &Id) -> Result<Option<User>, DatabaseError>;
    async fn user_from_username(&self, username: &Name) -> Result<Option<User>, DatabaseError>;
    async fn create_post(&mut self, data: CreatePost) -> Result<Post, DatabaseError>;
    async fn create_reply(&mut self, data: CreateReply) -> Result<Reply, DatabaseError>;
    async fn create_category(&mut self, data: CreateCategory) -> Result<Category, DatabaseError>;
    async fn category_from_id(&self, id: &Id) -> Result<Option<Category>, DatabaseError>;
}
