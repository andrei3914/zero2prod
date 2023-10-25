-- Add migration script here
CREATE TABLE users(
    user_id uuid PRIMARY_KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);
