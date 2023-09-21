#![allow(dead_code)]

use derive_more::Display;
use serde::Deserialize;

macro_rules! impl_from_for_newtype {
    ($type: tt) => {
        impl From<String> for $type {
            fn from(value: String) -> Self {
                Self(value)
            }
        }
    };
}

#[derive(Deserialize, sqlx::Type, Display)]
#[sqlx(transparent)]
pub struct Id(pub String);
#[derive(Deserialize, sqlx::Type, Display)]
#[sqlx(transparent)]
pub struct Name(pub String);
#[derive(Deserialize, sqlx::Type, Display)]
#[sqlx(transparent)]
pub struct Password(pub String);
#[derive(Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Link(String);

impl_from_for_newtype!(Id);
impl_from_for_newtype!(Name);
impl_from_for_newtype!(Password);
impl_from_for_newtype!(Link);

#[derive(Deserialize, sqlx::Type, Display)]
pub enum Permission {
    Unverified,
    User,
    Admin,
    Root,
}

impl From<String> for Permission {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Unverified" => Permission::Unverified,
            "User" => Permission::User,
            "Admin" => Permission::Admin,
            "Root" => Permission::Root,
            _ => unreachable!("should be saved as above"),
        }
    }
}

#[derive(Deserialize)]
pub struct User {
    pub id: Id,
    pub username: Name,
    pub nickname: Name,
    pub password: Password,
    pub permission: Permission,
    pub avatar_id: Option<Id>,
    pub date_created: String,
}

#[derive(Deserialize)]
pub struct Category {
    pub id: Id,
    pub name: Name,
    pub minimum_write_permission: Permission,
    pub minimum_read_permission: Permission,
    pub date_created: String,
}

#[derive(Deserialize)]
pub struct Post {
    pub id: Id,
    pub category_id: Id,
    pub title: Name,
    pub content: String,
    pub creator_id: Id,
    pub date_created: String,
}

#[derive(Deserialize)]
pub struct Reply {
    id: Id,
    creator_id: Id,
    post_id: Id,
    content: String,
    date_created: String,
}

#[derive(Deserialize)]
pub struct Attachment {
    id: Id,
    path: String,
    date_created: String,
}
