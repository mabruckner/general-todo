-- Add up migration script here
CREATE TABLE tasks (
    id  int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id int REFERENCES users(id),
    contents varchar(256),
    completed boolean
);
