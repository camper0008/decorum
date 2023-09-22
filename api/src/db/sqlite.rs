#![allow(unused_variables)]

use eyre::Context;
use sqlx::SqlitePool;

use crate::{
    from_unchecked::FromUnchecked, iso_date_strings::utc_date_iso_string, password::HashedPassword,
};

use super::{
    database::{CreateCategory, CreatePost, CreateReply, CreateUser, Database, DatabaseError},
    models::{Category, Content, Id, Name, Post, Reply, Title, User},
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
        let id = Id::new();
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
            id,
            username: data.username,
            nickname: data.nickname,
            password: data.password,
            permission: data.permission,
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
            id: Id::from_unchecked(user.id),
            username: Name::from_unchecked(user.username),
            nickname: user.nickname.map(Name::from_unchecked),
            password: HashedPassword::from_unchecked(user.password),
            permission: user.permission.into(),
            date_created: user.date_created,
            avatar_id: user.avatar_id.map(Id::from_unchecked),
        }))
    }
    async fn user_from_username(&self, username: &Name) -> Result<Option<User>, DatabaseError> {
        let user = sqlx::query!("SELECT * FROM user WHERE username=?;", username)
            .fetch_optional(&self.pool)
            .await
            .with_context(|| format!("unable to get user with username='{username}'"))?;

        let user = match user {
            Some(user) => user,
            None => return Ok(None),
        };

        let id = Id::from_unchecked(user.id);
        let username = Name::from_unchecked(user.username);
        let nickname = user.nickname.map(Name::from_unchecked);
        let password = HashedPassword::from_unchecked(user.password);
        let avatar_id = user.avatar_id.map(Id::from_unchecked);

        Ok(Some(User {
            id,
            username,
            nickname,
            password,
            avatar_id,
            permission: user.permission.into(),
            date_created: user.date_created.into(),
        }))
    }
    async fn create_post(&mut self, data: CreatePost) -> Result<Post, DatabaseError> {
        let id = Id::new();
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
            id,
            title: data.title,
            content: data.content,
            category_id: data.category_id,
            creator_id: data.creator_id,
            date_created,
        })
    }

    async fn create_reply(&mut self, data: CreateReply) -> Result<Reply, DatabaseError> {
        let id = Id::new();
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
            id,
            content: data.content,
            creator_id: data.creator_id,
            post_id: data.post_id,
            date_created,
        })
    }

    async fn create_category(&mut self, data: CreateCategory) -> Result<Category, DatabaseError> {
        let id = Id::new();
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
            id,
            title: data.title,
            minimum_read_permission: data.minimum_read_permission,
            minimum_write_permission: data.minimum_write_permission,
            date_created,
        })
    }
    async fn category_from_id(&self, id: &Id) -> Result<Option<Category>, DatabaseError> {
        let category = sqlx::query!("SELECT * FROM category WHERE id=?;", id)
            .fetch_optional(&self.pool)
            .await
            .with_context(|| format!("unable to get category with id='{id}'"))?;

        let category = match category {
            Some(category) => category,
            None => return Ok(None),
        };

        Ok(Some(Category {
            id: Id::from_unchecked(category.id),
            title: Title::from_unchecked(category.title),
            minimum_read_permission: category.minimum_read_permission.into(),
            minimum_write_permission: category.minimum_write_permission.into(),
            date_created: category.date_created,
        }))
    }

    async fn posts_from_category(&self, id: &Id) -> Result<Vec<Post>, DatabaseError> {
        let posts = sqlx::query!("SELECT * FROM post WHERE category_id=?;", id)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("unable to get posts"))?;

        Ok(posts
            .into_iter()
            .map(|post| Post {
                id: Id::from_unchecked(post.id),
                category_id: Id::from_unchecked(post.category_id),
                title: Title::from_unchecked(post.title),
                content: Content::from_unchecked(post.content),
                creator_id: Id::from_unchecked(post.creator_id),
                date_created: post.date_created,
            })
            .collect())
    }

    async fn all_categories(&self) -> Result<Vec<Category>, DatabaseError> {
        let categories = sqlx::query!("SELECT * FROM category;")
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("unable to get categories"))?;

        Ok(categories
            .into_iter()
            .map(|category| Category {
                id: Id::from_unchecked(category.id),
                title: Title::from_unchecked(category.title),
                minimum_read_permission: category.minimum_read_permission.into(),
                minimum_write_permission: category.minimum_write_permission.into(),
                date_created: category.date_created,
            })
            .collect())
    }

    async fn post_from_id(&self, id: &Id) -> Result<Option<Post>, DatabaseError> {
        let post = sqlx::query!("SELECT * FROM post WHERE id=?;", id)
            .fetch_optional(&self.pool)
            .await
            .with_context(|| format!("unable to get post with id='{id}'"))?;

        let post = match post {
            Some(post) => post,
            None => return Ok(None),
        };

        Ok(Some(Post {
            id: Id::from_unchecked(post.id),
            category_id: Id::from_unchecked(post.category_id),
            title: Title::from_unchecked(post.title),
            content: Content::from_unchecked(post.content),
            creator_id: Id::from_unchecked(post.creator_id),
            date_created: post.date_created,
        }))
    }

    async fn replies_from_post(&self, id: &Id) -> Result<Vec<Reply>, DatabaseError> {
        let posts = sqlx::query!("SELECT * FROM reply WHERE post_id=?;", id)
            .fetch_all(&self.pool)
            .await
            .with_context(|| format!("unable to get replies"))?;

        Ok(posts
            .into_iter()
            .map(|reply| Reply {
                id: Id::from_unchecked(reply.id),
                content: Content::from_unchecked(reply.content),
                creator_id: Id::from_unchecked(reply.creator_id),
                post_id: Id::from_unchecked(reply.post_id),
                date_created: reply.date_created,
            })
            .collect())
    }
}
