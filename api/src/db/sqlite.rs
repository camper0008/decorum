#![allow(unused_variables)]

use eyre::Context;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::iso_date_strings::utc_date_iso_string;

use super::{
    database::{CreateCategory, CreatePost, CreateReply, CreateUser, Database, DatabaseError},
    models::{Category, Id, Permission, Post, Reply, User},
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
            id: id.into(),
            username: data.username.into(),
            nickname: data.nickname.into(),
            password: data.password.into(),
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
            id: user.id.into(),
            username: user.username.into(),
            nickname: user.nickname.into(),
            password: user.password.into(),
            permission: user.permission.into(),
            date_created: user.date_created,
            avatar_id: user.avatar_id.map(|id| id.into()),
        }))
    }
    async fn user_from_username(&self, username: &String) -> Result<Option<User>, DatabaseError> {
        let user = sqlx::query!("SELECT * FROM user WHERE username=?;", username)
            .fetch_optional(&self.pool)
            .await
            .with_context(|| format!("unable to get user with username='{username}'"))?;
        Ok(user.map(|user| User {
            id: user.id.into(),
            username: user.username.into(),
            nickname: user.nickname.into(),
            password: user.password.into(),
            permission: user.permission.into(),
            date_created: user.date_created,
            avatar_id: user.avatar_id.map(|id| id.into()),
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
            id: id.into(),
            title: data.title.into(),
            content: data.content.into(),
            category_id: data.category_id,
            creator_id: data.creator_id,
            date_created,
        })
    }

    async fn create_reply(&mut self, data: CreateReply) -> Result<Reply, DatabaseError> {
        let id = Uuid::new_v4().to_string();
        let date_created = utc_date_iso_string();

        sqlx::query!(
            "INSERT INTO reply (id, content, creator_id, post_id, date_created) VALUES (?, ?, ?, ?, ?);",
            id,
            data.content,
            data.creator_id,
            data.post_id,
            date_created,
        )
        .execute(&self.pool)
        .await
        .with_context(|| "unable to insert reply")?;

        Ok(Reply {
            id: id.into(),
            content: data.content.into(),
            creator_id: data.creator_id.into(),
            post_id: data.post_id.into(),
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
        .with_context(|| "unable to insert category")?;

        Ok(Category {
            id: id.into(),
            title: data.title.into(),
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
