use std::sync::Arc;

use salvo::async_trait;
use tokio::sync::RwLock;

use super::models::{Category, Id, Permission, Post, User};

pub type DatabaseError = eyre::Report;

pub type DatabaseParam = Arc<RwLock<dyn Database + Send + Sync>>;

pub struct CreateUser {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub permission: Permission,
    pub avatar_id: Option<Id>,
}

pub struct CreatePost {
    pub category_id: Id,
    pub title: String,
    pub content: String,
    pub creator_id: Id,
}

#[async_trait]
pub trait Database {
    async fn create_user(&mut self, data: CreateUser) -> Result<User, DatabaseError>;
    async fn delete_user_with_id(&mut self, id: &Id) -> Result<Option<User>, DatabaseError>;
    async fn create_post(&mut self, data: CreatePost) -> Result<Post, DatabaseError>;
    async fn user_from_id(&self, id: &Id) -> Result<Option<User>, DatabaseError>;
    async fn user_from_username(&self, username: &String) -> Result<Option<User>, DatabaseError>;
    async fn category_from_id(&self, id: &Id) -> Result<Option<Category>, DatabaseError>;
}
