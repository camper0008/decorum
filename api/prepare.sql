DROP TABLE IF EXISTS user;
CREATE TABLE user (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    nickname TEXT NOT NULL,
    password TEXT NOT NULL,
    permission TEXT NOT NULL,
    date_created TEXT NOT NULL,
    avatar: VARCHAR(36)
);

DROP TABLE IF EXISTS category;
CREATE TABLE category (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    minimum_write_permission TEXT NOT NULL,
    minimum_read_permission TEXT NOT NULL,
    date_created TEXT NOT NULL
);

DROP TABLE IF EXISTS post;
CREATE TABLE post (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    date_created TEXT NOT NULL,
    creator_id VARCHAR(36) NOT NULL
);

DROP TABLE IF EXISTS reply;
CREATE TABLE reply (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    creator_id VARCHAR(36) NOT NULL,
    post_id VARCHAR(36) NOT NULL,
    date_created TEXT NOT NULL
);


DROP TABLE IF EXISTS attachment;
CREATE TABLE attachment (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    path TEXT NOT NULL,
    date_created TEXT NOT NULL
);