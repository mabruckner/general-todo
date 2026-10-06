-- Add up migration script here
CREATE TABLE users (
    id  int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    username varchar(80) UNIQUE NOT NULL,
    pass_hash varchar(200)
);
