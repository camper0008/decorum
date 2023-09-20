#![allow(dead_code)]

use derive_more::Display;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Id(String);

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
    id: Id,
    username: String,
    nickname: String,
    password: String,
    pub permission: Permission,
    avatar: Option<Attachment>,
}

#[derive(Deserialize)]
pub struct Category {
    id: Id,
    pub name: String,
    pub minimum_permission: Permission,
}

#[derive(Deserialize)]
pub struct Post {
    id: Id,
    category: Id,
    title: String,
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
