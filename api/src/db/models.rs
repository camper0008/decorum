pub struct Id(String);
pub struct Link(String);

pub enum Permission {
    Visitor,
    User,
    Admin,
    Root,
}

pub struct User {
    id: Id,
    username: String,
    nickname: String,
    password: String,
    avatar: Option<Attachment>,
}

pub struct Category {
    id: Id,
    name: String,
}

pub struct Post {
    id: Id,
    category: Id,
    title: String,
    content: String,
    creator_id: String,
}

pub struct Reply {
    id: Id,
    creator_id: Id,
    post_id: Id,
    content: String,
}

pub struct Attachment {
    id: Id,
    path: String,
}
