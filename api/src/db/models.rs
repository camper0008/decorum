#![allow(dead_code)]

use derive_more::Display;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Id(pub String);
#[derive(Deserialize)]
pub struct Name(pub String);
#[derive(Deserialize)]
pub struct Password(pub String);

impl From<String> for Id {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Deserialize)]
pub struct Link(String);

#[derive(Deserialize, Display)]
pub enum Permission {
    Visitor,
    User,
    Admin,
    Root,
}

#[derive(Deserialize)]
pub struct User {
    pub id: Id,
    username: Name,
    nickname: Name,
    pub password: Password,
    pub permission: Permission,
    avatar: Option<Id>,
}

#[derive(Deserialize)]
pub struct Category {
    id: Id,
    pub name: Name,
    pub minimum_write_permission: Permission,
    pub minimum_read_permission: Permission,
}

#[derive(Deserialize)]
pub struct Post {
    id: Id,
    category: Id,
    title: Name,
    content: String,
    creator_id: Id,
}

#[derive(Deserialize)]
pub struct Reply {
    id: Id,
    creator_id: Id,
    post_id: Id,
    content: String,
}

#[derive(Deserialize)]
pub struct Attachment {
    id: Id,
    path: String,
}
