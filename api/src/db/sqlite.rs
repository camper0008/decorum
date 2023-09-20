#![allow(unused_variables)]

use super::{
    database::{Database, DatabaseError},
    models::{Attachment, Category, Id, Post, User},
};

pub struct SqliteDb {}

#[salvo::async_trait]
impl Database for SqliteDb {
    async fn create_user(
        &mut self,
        username: String,
        nickname: String,
        password: String,
        avatar: Option<Attachment>,
    ) -> Result<User, DatabaseError> {
        todo!()
    }
    async fn delete_user(
        &mut self,
        username: String,
        nickname: String,
        password: String,
        avatar: Option<Attachment>,
    ) -> Result<User, DatabaseError> {
        todo!()
    }
    async fn create_post(
        &mut self,
        category: Id,
        title: String,
        content: String,
        creator_id: Id,
    ) -> Result<Post, DatabaseError> {
        todo!()
    }
    async fn user_from_id(&self, id: &Id) -> Result<User, DatabaseError> {
        todo!()
    }
    async fn category_from_id(&self, id: &Id) -> Result<Category, DatabaseError> {
        todo!()
    }
}
