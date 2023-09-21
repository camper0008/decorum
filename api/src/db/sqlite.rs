#![allow(unused_variables)]

use eyre::Context;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::iso_date_strings::utc_date_iso_string;

use super::{
    database::{CreateCategory, CreatePost, CreateUser, Database, DatabaseError},
    models::{Category, Id, Name, Password, Permission, Post, User},
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
        let id = Uuid::new_v4().to_string();
        let date_created = utc_date_iso_string();

        sqlx::query!(
            "INSERT INTO user (id, username, nickname, password, permission, avatar_id, date_created) VALUES (?, ?, ?, ?, ?, ?, ?);",
            id,
            data.username,
            data.nickname,
            data.password,
            data.permission,
            data.avatar_id,
            date_created,
        )
        .execute(&self.pool)
        .await
        .with_context(|| "unable to insert user")?;

        Ok(User {
            id: Id(id),
            username: Name(data.username),
            nickname: Name(data.nickname),
            password: Password(data.password),
            permission: Permission::Unverified,
            avatar_id: data.avatar_id,
            date_created,
        })
    }
    async fn delete_user_with_id(&mut self, id: &Id) -> Result<Option<User>, DatabaseError> {
        let user = self.user_from_id(id).await?;
        sqlx::query!("DELETE FROM user WHERE id=?;", id)
            .execute(&self.pool)
            .await
            .with_context(|| format!("unable to delete user with id='{id}'"))?;
        Ok(user)
    }
    async fn user_from_id(&self, id: &Id) -> Result<Option<User>, DatabaseError> {
        let user = sqlx::query!("SELECT * FROM user WHERE id=?;", id)
            .fetch_optional(&self.pool)
            .await
            .with_context(|| format!("unable to get user with id='{id}'"))?;
        Ok(user.map(|user| User {
            id: Id(user.id),
            username: Name(user.username),
            nickname: Name(user.nickname),
            password: Password(user.password),
            permission: user.permission.into(),
            date_created: user.date_created,
            avatar_id: user.avatar_id.map(Id),
        }))
    }
    async fn user_from_username(&self, username: &String) -> Result<Option<User>, DatabaseError> {
        let user = sqlx::query!("SELECT * FROM user WHERE username=?;", username)
            .fetch_optional(&self.pool)
            .await
            .with_context(|| format!("unable to get user with username='{username}'"))?;
        Ok(user.map(|user| User {
            id: Id(user.id),
            username: Name(user.username),
            nickname: Name(user.nickname),
            password: Password(user.password),
            permission: user.permission.into(),
            date_created: user.date_created,
            avatar_id: user.avatar_id.map(Id),
        }))
    }
    async fn create_post(&mut self, data: CreatePost) -> Result<Post, DatabaseError> {
        let id = Uuid::new_v4().to_string();
        let date_created = utc_date_iso_string();

        sqlx::query!(
            "INSERT INTO post (id, title, content, category_id, creator_id, date_created) VALUES (?, ?, ?, ?, ?, ?);",
            id,
            data.title,
            data.content,
            data.category_id,
            data.creator_id,
            date_created,
        )
        .execute(&self.pool)
        .await
        .with_context(|| "unable to insert post")?;

        Ok(Post {
            id: Id(id),
            title: Name(data.title),
            content: data.content,
            category_id: data.category_id,
            creator_id: data.creator_id,
            date_created,
        })
    }
    async fn create_category(&mut self, data: CreateCategory) -> Result<Category, DatabaseError> {
        let id = Uuid::new_v4().to_string();
        let date_created = utc_date_iso_string();

        sqlx::query!(
            "INSERT INTO category (id, title, minimum_write_permission, minimum_read_permission, date_created) VALUES (?, ?, ?, ?, ?);",
            id,
            data.title,
            data.minimum_write_permission,
            data.minimum_read_permission,
            date_created,
        )
        .execute(&self.pool)
        .await
        .with_context(|| "unable to insert post")?;

        Ok(Category {
            id: Id(id),
            title: Name(data.title),
            minimum_read_permission: data.minimum_read_permission,
            minimum_write_permission: data.minimum_write_permission,
            date_created,
        })
    }
    async fn category_from_id(&self, id: &Id) -> Result<Option<Category>, DatabaseError> {
        sqlx::query_as!(Category, "SELECT * FROM category WHERE id=?;", id)
            .fetch_optional(&self.pool)
            .await
            .with_context(|| format!("unable to get user with id='{id}'"))
    }
}
