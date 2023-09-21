#![allow(unused_variables)]

use eyre::Context;
use sqlx::SqlitePool;

use super::{
    database::{CreatePost, CreateUser, Database, DatabaseError},
    models::{Category, Id, Post, User},
};

pub struct SqliteDb {
    pool: SqlitePool,
}

impl SqliteDb {
    pub async fn new(db_url: String) -> Result<Self, DatabaseError> {
        let pool = SqlitePool::connect(&db_url)
            .await
            .with_context(|| "unable to connect to database")?;

        Ok(Self { pool })
    }
}

#[salvo::async_trait]
impl Database for SqliteDb {
    async fn create_user(&mut self, data: CreateUser) -> Result<User, DatabaseError> {
        todo!()
    }
    async fn delete_user_with_id(&mut self, id: &Id) -> Result<Option<User>, DatabaseError> {
        todo!()
    }
    async fn create_post(&mut self, data: CreatePost) -> Result<Post, DatabaseError> {
        todo!()
    }
    async fn user_from_id(&self, id: &Id) -> Result<Option<User>, DatabaseError> {
        todo!()
    }
    async fn category_from_id(&self, id: &Id) -> Result<Option<Category>, DatabaseError> {
        todo!()
    }
    async fn user_from_username(&self, username: &String) -> Result<Option<User>, DatabaseError> {
        todo!()
    }
}
