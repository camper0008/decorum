use std::sync::Arc;

use salvo::async_trait;
use tokio::sync::RwLock;

use crate::password::HashedPassword;

use super::models::{Category, Content, Id, Name, Permission, Post, Reply, Title, User};

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
    pub title: Title,
    pub content: Content,
    pub creator_id: Id,
}

pub struct EditPost {
    pub id: Id,
    pub category_id: Id,
    pub title: Title,
    pub content: Content,
    pub deleted: bool,
    pub locked: bool,
}

pub struct CreateCategory {
    pub title: Title,
    pub minimum_write_permission: Permission,
    pub minimum_read_permission: Permission,
}

pub struct EditCategory {
    pub id: Id,
    pub title: Title,
    pub minimum_write_permission: Permission,
    pub minimum_read_permission: Permission,
    pub deleted: bool,
}

pub struct CreateReply {
    pub creator_id: Id,
    pub post_id: Id,
    pub content: Content,
}

pub struct EditReply {
    pub id: Id,
    pub content: Content,
    pub deleted: bool,
}

#[async_trait]
pub trait Database {
    async fn create_user(&mut self, data: CreateUser) -> Result<(), DatabaseError>;
    async fn create_category(&mut self, data: CreateCategory) -> Result<(), DatabaseError>;
    async fn create_post(&mut self, data: CreatePost) -> Result<(), DatabaseError>;
    async fn create_reply(&mut self, data: CreateReply) -> Result<(), DatabaseError>;
    async fn user_from_id(&self, id: &Id) -> Result<Option<User>, DatabaseError>;
    async fn user_from_username(&self, username: &Name) -> Result<Option<User>, DatabaseError>;
    async fn category_from_id(&self, id: &Id) -> Result<Option<Category>, DatabaseError>;
    async fn all_categories(&self) -> Result<Vec<Category>, DatabaseError>;
    async fn post_from_id(&self, id: &Id) -> Result<Option<Post>, DatabaseError>;
    async fn posts_from_category(&self, id: &Id) -> Result<Vec<Post>, DatabaseError>;
    async fn replies_from_post(&self, id: &Id) -> Result<Vec<Reply>, DatabaseError>;
    async fn reply_from_id(&self, id: &Id) -> Result<Option<Reply>, DatabaseError>;
    async fn delete_user_with_id(&mut self, id: &Id) -> Result<(), DatabaseError>;
    async fn edit_category(&mut self, data: EditCategory) -> Result<(), DatabaseError>;
    async fn edit_post(&mut self, data: EditPost) -> Result<(), DatabaseError>;
    async fn edit_reply(&mut self, data: EditReply) -> Result<(), DatabaseError>;
}
