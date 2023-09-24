#![allow(unused_variables)]

use eyre::Context;
use sqlx::SqlitePool;

use crate::{
    from_unchecked::FromUnchecked, iso_date_strings::utc_date_iso_string, password::HashedPassword,
};

use super::{
    database::{
        CreateCategory, CreatePost, CreateReply, CreateUser, Database, DatabaseError, EditCategory,
        EditPost, EditReply, EditUser,
    },
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
    async fn create_user(&mut self, data: CreateUser) -> Result<(), DatabaseError> {
        let id = Id::new();
        let date_created = utc_date_iso_string();

        sqlx::query!(
            "INSERT INTO user (id, username, nickname, password, permission, avatar_id, deleted, date_created) VALUES (?, ?, ?, ?, ?, ?, ?, ?);",
            id,
            data.username,
            data.nickname,
            data.password,
            data.permission,
            data.avatar_id,
            false,
            date_created,
        )
        .execute(&self.pool)
        .await
        .with_context(|| "unable to insert user")?;

        Ok(())
    }
    async fn delete_user_with_id(&mut self, id: &Id) -> Result<(), DatabaseError> {
        let user = self.user_from_id(id).await?;
        sqlx::query!("DELETE FROM user WHERE id=?;", id)
            .execute(&self.pool)
            .await
            .with_context(|| format!("unable to delete user with id='{id}'"))?;
        Ok(())
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
            avatar_id: user.avatar_id.map(Id::from_unchecked),
            deleted: user.deleted != 0,
            date_created: user.date_created,
            date_edited: user.date_edited,
        }))
    }
    async fn user_from_username(&self, username: &Name) -> Result<Option<User>, DatabaseError> {
        let user = sqlx::query!("SELECT * FROM user WHERE username=?;", username)
            .fetch_optional(&self.pool)
            .await
            .with_context(|| format!("unable to get user with username='{username}'"))?;

        Ok(user.map(|user| User {
            id: Id::from_unchecked(user.id),
            username: Name::from_unchecked(user.username),
            nickname: user.nickname.map(Name::from_unchecked),
            password: HashedPassword::from_unchecked(user.password),
            avatar_id: user.avatar_id.map(Id::from_unchecked),
            permission: user.permission.into(),
            deleted: user.deleted != 0,
            date_edited: user.date_edited,
            date_created: user.date_created,
        }))
    }
    async fn create_post(&mut self, data: CreatePost) -> Result<(), DatabaseError> {
        let id = Id::new();
        let date_created = utc_date_iso_string();

        sqlx::query!(
            "INSERT INTO post (id, title, content, category_id, creator_id, locked, deleted, date_created) VALUES (?, ?, ?, ?, ?, ?, ?, ?);",
            id,
            data.title,
            data.content,
            data.category_id,
            data.creator_id,
            false,
            false,
            date_created,
        )
        .execute(&self.pool)
        .await
        .with_context(|| "unable to insert post")?;

        Ok(())
    }

    async fn create_reply(&mut self, data: CreateReply) -> Result<(), DatabaseError> {
        let id = Id::new();
        let date_created = utc_date_iso_string();

        sqlx::query!(
            "INSERT INTO reply (id, content, creator_id, post_id, deleted, date_created) VALUES (?, ?, ?, ?, ?, ?);",
            id,
            data.content,
            data.creator_id,
            data.post_id,
            false,
            date_created,
        )
        .execute(&self.pool)
        .await
        .with_context(|| "unable to insert reply")?;

        Ok(())
    }

    async fn create_category(&mut self, data: CreateCategory) -> Result<(), DatabaseError> {
        let id = Id::new();
        let date_created = utc_date_iso_string();

        sqlx::query!(
            "INSERT INTO category (id, title, minimum_write_permission, minimum_read_permission, deleted, date_created) VALUES (?, ?, ?, ?, ?, ?);",
            id,
            data.title,
            data.minimum_write_permission,
            data.minimum_read_permission,
            false,
            date_created,
        )
        .execute(&self.pool)
        .await
        .with_context(|| "unable to insert category")?;

        Ok(())
    }

    async fn edit_user(&mut self, data: EditUser) -> Result<(), DatabaseError> {
        let date_edited = utc_date_iso_string();

        sqlx::query!(
            "UPDATE user SET nickname=?, password=?, permission=?, avatar_id=?, deleted=? WHERE id=?;",
            data.nickname,
            data.password,
            data.permission,
            data.avatar_id,
            data.deleted,
            data.id,
        )
        .execute(&self.pool)
        .await
        .with_context(|| "unable to edit category")?;

        Ok(())
    }

    async fn edit_category(&mut self, data: EditCategory) -> Result<(), DatabaseError> {
        let date_edited = utc_date_iso_string();

        sqlx::query!(
            "UPDATE category SET title=?, minimum_read_permission=?, minimum_write_permission=?, deleted=?, date_edited=? WHERE id=?;",
            data.title,
            data.minimum_read_permission,
            data.minimum_write_permission,
            data.deleted,
            date_edited,
            data.id,
        )
        .execute(&self.pool)
        .await
        .with_context(|| "unable to edit category")?;

        Ok(())
    }

    async fn edit_post(&mut self, data: EditPost) -> Result<(), DatabaseError> {
        let date_edited = utc_date_iso_string();

        sqlx::query!(
            "UPDATE post SET title=?, content=?, category_id=?, date_edited=?, deleted=?, locked=? WHERE id=?;",
            data.title,
            data.content,
            data.category_id,
            date_edited,
            data.deleted,
            data.locked,
            data.id,
        )
        .execute(&self.pool)
        .await
        .with_context(|| "unable to edit post")?;

        Ok(())
    }

    async fn edit_reply(&mut self, data: EditReply) -> Result<(), DatabaseError> {
        let date_edited = utc_date_iso_string();

        sqlx::query!(
            "UPDATE reply SET content=?, deleted=?, date_edited=? WHERE id=?;",
            data.content,
            data.deleted,
            date_edited,
            data.id,
        )
        .execute(&self.pool)
        .await
        .with_context(|| "unable to reply post")?;

        Ok(())
    }

    async fn category_from_id(&self, id: &Id) -> Result<Option<Category>, DatabaseError> {
        let category = sqlx::query!("SELECT * FROM category WHERE id=?;", id)
            .fetch_optional(&self.pool)
            .await
            .with_context(|| format!("unable to get category with id='{id}'"))?;

        Ok(category.map(|category| Category {
            id: Id::from_unchecked(category.id),
            title: Title::from_unchecked(category.title),
            minimum_read_permission: category.minimum_read_permission.into(),
            minimum_write_permission: category.minimum_write_permission.into(),
            date_created: category.date_created,
            date_edited: category.date_edited,
            deleted: category.deleted != 0,
        }))
    }

    async fn posts_from_category(&self, id: &Id) -> Result<Vec<Post>, DatabaseError> {
        let posts = sqlx::query!("SELECT * FROM post WHERE category_id=?;", id)
            .fetch_all(&self.pool)
            .await
            .with_context(|| "unable to get posts")?;

        Ok(posts
            .into_iter()
            .map(|post| Post {
                id: Id::from_unchecked(post.id),
                category_id: Id::from_unchecked(post.category_id),
                title: Title::from_unchecked(post.title),
                content: Content::from_unchecked(post.content),
                creator_id: Id::from_unchecked(post.creator_id),
                date_created: post.date_created,
                date_edited: post.date_edited,
                deleted: post.deleted != 0,
                locked: post.locked != 0,
            })
            .collect())
    }

    async fn all_categories(&self) -> Result<Vec<Category>, DatabaseError> {
        let categories = sqlx::query!("SELECT * FROM category;")
            .fetch_all(&self.pool)
            .await
            .with_context(|| "unable to get categories")?;

        Ok(categories
            .into_iter()
            .map(|category| Category {
                id: Id::from_unchecked(category.id),
                title: Title::from_unchecked(category.title),
                minimum_read_permission: category.minimum_read_permission.into(),
                minimum_write_permission: category.minimum_write_permission.into(),
                date_created: category.date_created,
                date_edited: category.date_edited,
                deleted: category.deleted != 0,
            })
            .collect())
    }

    async fn post_from_id(&self, id: &Id) -> Result<Option<Post>, DatabaseError> {
        let post = sqlx::query!("SELECT * FROM post WHERE id=?;", id)
            .fetch_optional(&self.pool)
            .await
            .with_context(|| format!("unable to get post with id='{id}'"))?;

        Ok(post.map(|post| Post {
            id: Id::from_unchecked(post.id),
            category_id: Id::from_unchecked(post.category_id),
            title: Title::from_unchecked(post.title),
            content: Content::from_unchecked(post.content),
            creator_id: Id::from_unchecked(post.creator_id),
            date_created: post.date_created,
            date_edited: post.date_edited,
            deleted: post.deleted != 0,
            locked: post.locked != 0,
        }))
    }

    async fn replies_from_post(&self, id: &Id) -> Result<Vec<Reply>, DatabaseError> {
        let posts = sqlx::query!("SELECT * FROM reply WHERE post_id=?;", id)
            .fetch_all(&self.pool)
            .await
            .with_context(|| "unable to get replies")?;

        Ok(posts
            .into_iter()
            .map(|reply| Reply {
                id: Id::from_unchecked(reply.id),
                content: Content::from_unchecked(reply.content),
                creator_id: Id::from_unchecked(reply.creator_id),
                post_id: Id::from_unchecked(reply.post_id),
                date_created: reply.date_created,
                date_edited: reply.date_edited,
                deleted: reply.deleted != 0,
            })
            .collect())
    }

    async fn reply_from_id(&self, id: &Id) -> Result<Option<Reply>, DatabaseError> {
        let post = sqlx::query!("SELECT * FROM reply WHERE id=?;", id)
            .fetch_optional(&self.pool)
            .await
            .with_context(|| "unable to get reply")?;

        Ok(post.map(|reply| Reply {
            id: Id::from_unchecked(reply.id),
            content: Content::from_unchecked(reply.content),
            creator_id: Id::from_unchecked(reply.creator_id),
            post_id: Id::from_unchecked(reply.post_id),
            date_created: reply.date_created,
            date_edited: reply.date_edited,
            deleted: reply.deleted != 0,
        }))
    }
}
