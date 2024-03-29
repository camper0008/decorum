DROP TABLE IF EXISTS user;
CREATE TABLE user (
    id VARCHAR(16) PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    nickname TEXT,
    password TEXT NOT NULL,
    permission TEXT NOT NULL,
    avatar_id VARCHAR(8),
    date_edited TEXT,
    date_created TEXT NOT NULL,
    deleted INTEGER not null,
    FOREIGN KEY(avatar_id) REFERENCES attachment(id)
);

DROP TABLE IF EXISTS category;
CREATE TABLE category (
    id VARCHAR(8) PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    minimum_write_permission TEXT NOT NULL,
    minimum_read_permission TEXT NOT NULL,
    deleted INTEGER not null,
    date_edited TEXT,
    date_created TEXT NOT NULL
);

DROP TABLE IF EXISTS post;
CREATE TABLE post (
    id VARCHAR(8) PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    category_id VARCHAR(8) NOT NULL,
    creator_id VARCHAR(8) NOT NULL,
    locked INTEGER not null,
    deleted INTEGER not null,
    date_created TEXT NOT NULL,
    date_edited TEXT,
    FOREIGN KEY(creator_id) REFERENCES user(id)
    FOREIGN KEY(category_id) REFERENCES category(id)
);

DROP TABLE IF EXISTS reply;
CREATE TABLE reply (
    id VARCHAR(8) PRIMARY KEY NOT NULL,
    creator_id VARCHAR(8) NOT NULL,
    content TEXT NOT NULL,
    post_id VARCHAR(8) NOT NULL,
    date_edited TEXT,
    date_created TEXT NOT NULL,
    deleted INTEGER not null,
    FOREIGN KEY(post_id) REFERENCES post(id),
    FOREIGN KEY(creator_id) REFERENCES user(id)
);


DROP TABLE IF EXISTS attachment;
CREATE TABLE attachment (
    id VARCHAR(8) PRIMARY KEY NOT NULL,
    path TEXT NOT NULL,
    creator_id VARCHAR(8) NOT NULL,
    date_created TEXT NOT NULL,
    FOREIGN KEY(creator_id) REFERENCES user(id)
);
