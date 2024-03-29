use std::{path::PathBuf, sync::Arc};

use salvo::async_trait;
use tokio::sync::RwLock;

use crate::password::HashedPassword;

use super::models::{
    Attachment, Category, Content, Id, Name, Permission, Post, Reply, Title, User,
};

pub type DatabaseError = eyre::Report;

pub type DatabaseParam = Arc<RwLock<dyn Database + Send + Sync>>;

pub struct CreateUser {
    pub username: Name,
    pub nickname: Option<Name>,
    pub password: HashedPassword,
    pub permission: Permission,
    pub avatar_id: Option<Id>,
}

pub struct CreateAttachment<'a> {
    pub creator_id: Id,
    pub file_name: &'a str,
    pub temp_path: &'a PathBuf,
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

pub struct EditUser {
    pub id: Id,
    pub nickname: Option<Name>,
    pub password: HashedPassword,
    pub permission: Permission,
    pub avatar_id: Option<Id>,
    pub deleted: bool,
}

#[async_trait]
pub trait Database {
    async fn create_user(&mut self, data: CreateUser) -> Result<Id, DatabaseError>;
    async fn create_category(&mut self, data: CreateCategory) -> Result<Id, DatabaseError>;
    async fn create_post(&mut self, data: CreatePost) -> Result<Id, DatabaseError>;
    async fn create_reply(&mut self, data: CreateReply) -> Result<Id, DatabaseError>;
    async fn create_attachment<'a>(
        &mut self,
        data: CreateAttachment<'a>,
    ) -> Result<Id, DatabaseError>;
    async fn user_from_id(&self, id: &Id) -> Result<Option<User>, DatabaseError>;
    async fn user_from_username(&self, username: &Name) -> Result<Option<User>, DatabaseError>;
    async fn category_from_id(&self, id: &Id) -> Result<Option<Category>, DatabaseError>;
    async fn all_categories(&self) -> Result<Vec<Category>, DatabaseError>;
    async fn post_from_id(&self, id: &Id) -> Result<Option<Post>, DatabaseError>;
    async fn posts_from_category(&self, id: &Id) -> Result<Vec<Post>, DatabaseError>;
    async fn replies_from_post(&self, id: &Id) -> Result<Vec<Reply>, DatabaseError>;
    async fn reply_from_id(&self, id: &Id) -> Result<Option<Reply>, DatabaseError>;
    async fn attachment_from_id(&self, id: &Id) -> Result<Option<Attachment>, DatabaseError>;
    async fn edit_user(&mut self, data: EditUser) -> Result<(), DatabaseError>;
    async fn edit_category(&mut self, data: EditCategory) -> Result<(), DatabaseError>;
    async fn edit_post(&mut self, data: EditPost) -> Result<(), DatabaseError>;
    async fn edit_reply(&mut self, data: EditReply) -> Result<(), DatabaseError>;
}
