#![allow(dead_code)]

use crate::from_unchecked::FromUnchecked;
use derive_more::Display;
use salvo::prelude::ToSchema;
use serde::Deserialize;
use uuid::Uuid;

use crate::password::HashedPassword;

macro_rules! impl_from_for_newtype {
    ($type: tt, $length_range: expr) => {
        impl TryFrom<String> for $type {
            type Error = String;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if $length_range.contains(&value.len()) {
                    Ok(Self(value))
                } else {
                    Err("length invalid".to_string())
                }
            }
        }

        impl FromUnchecked<String> for $type {
            fn from_unchecked(v: String) -> Self {
                Self(v)
            }
        }
    };
}

#[derive(Deserialize, sqlx::Type, Display)]
#[sqlx(transparent)]
pub struct Id(String);
#[derive(Deserialize, sqlx::Type, Display)]
#[sqlx(transparent)]
pub struct Content(String);
#[derive(Deserialize, sqlx::Type, Display)]
#[sqlx(transparent)]
pub struct Name(String);
#[derive(Deserialize, sqlx::Type, Display)]
#[sqlx(transparent)]
pub struct Link(String);

impl_from_for_newtype!(Id, 36..=36);
impl_from_for_newtype!(Content, 25..=1000);
impl_from_for_newtype!(Name, 1..=32);
impl_from_for_newtype!(Link, 1..);

#[derive(Deserialize, sqlx::Type, Display, ToSchema)]
pub enum Permission {
    Unverified,
    User,
    Admin,
    Root,
}

impl Id {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
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
    pub nickname: Option<Name>,
    pub password: HashedPassword,
    pub permission: Permission,
    pub avatar_id: Option<Id>,
    pub date_created: String,
}

#[derive(Deserialize)]
pub struct Category {
    pub id: Id,
    pub title: Name,
    pub minimum_write_permission: Permission,
    pub minimum_read_permission: Permission,
    pub date_created: String,
}

#[derive(Deserialize)]
pub struct Post {
    pub id: Id,
    pub category_id: Id,
    pub title: Name,
    pub content: Content,
    pub creator_id: Id,
    pub date_created: String,
}

#[derive(Deserialize)]
pub struct Reply {
    pub id: Id,
    pub creator_id: Id,
    pub post_id: Id,
    pub content: Content,
    pub date_created: String,
}

#[derive(Deserialize)]
pub struct Attachment {
    id: Id,
    path: String,
    date_created: String,
}
