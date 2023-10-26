use crate::from_unchecked::FromUnchecked;
use derive_more::Display;
use salvo::{oapi, prelude::ToSchema, writing::Json, Scribe};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::password::HashedPassword;

macro_rules! define_newtype {
    ($name: tt, $length_range: expr) => {
        #[must_use]
        #[derive(Serialize, Deserialize, sqlx::Type, Display, oapi::ToSchema, PartialEq)]
        #[sqlx(transparent)]
        pub struct $name(String);

        impl TryFrom<String> for $name {
            type Error = String;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if $length_range.contains(&value.len()) {
                    Ok(Self(value))
                } else {
                    Err("length invalid".to_string())
                }
            }
        }

        impl FromUnchecked<String> for $name {
            fn from_unchecked(v: String) -> Self {
                Self(v)
            }
        }
    };
}

define_newtype!(Id, 8..=8);
define_newtype!(Content, 1..=1024);
define_newtype!(Name, 1..=32);
define_newtype!(Title, 1..=128);
define_newtype!(Link, 1..);

macro_rules! impl_json_writer {
    ($name: ident) => {
        impl Scribe for $name {
            fn render(self, res: &mut salvo::Response) {
                res.render(Json(self))
            }
        }
    };
}

#[derive(Serialize, Deserialize, sqlx::Type, Display, ToSchema)]
pub enum Permission {
    Banned,
    Unverified,
    User,
    Admin,
    Root,
}

impl Default for Permission {
    fn default() -> Self {
        Permission::Unverified
    }
}

impl Id {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string()[0..8].to_string())
    }
}

impl From<String> for Permission {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Banned" => Permission::Banned,
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
    pub deleted: bool,
    pub date_created: String,
    pub date_edited: Option<String>,
}

#[derive(Serialize, Deserialize, oapi::ToSchema)]
pub struct Category {
    pub id: Id,
    pub title: Title,
    pub minimum_write_permission: Permission,
    pub minimum_read_permission: Permission,
    pub deleted: bool,
    pub date_created: String,
    pub date_edited: Option<String>,
}
impl_json_writer!(Category);

#[derive(Serialize, Deserialize, oapi::ToSchema)]
pub struct Post {
    pub id: Id,
    pub category_id: Id,
    pub title: Title,
    pub content: Content,
    pub creator_id: Id,
    pub deleted: bool,
    pub locked: bool,
    pub date_created: String,
    pub date_edited: Option<String>,
}
impl_json_writer!(Post);

#[derive(Deserialize, Serialize, oapi::ToSchema)]
pub struct Reply {
    pub id: Id,
    pub creator_id: Id,
    pub post_id: Id,
    pub content: Content,
    pub deleted: bool,
    pub date_created: String,
    pub date_edited: Option<String>,
}
impl_json_writer!(Reply);

#[derive(Deserialize)]
pub struct Attachment {
    pub id: Id,
    pub path: String,
    pub creator_id: Id,
    pub date_created: String,
}
