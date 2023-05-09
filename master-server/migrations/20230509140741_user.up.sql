-- Add up migration script here
-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    uuid UUID PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    token VARCHAR NOT NULL UNIQUE,
    refresh_token VARCHAR NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    status INTEGER NOT NULL DEFAULT 0
);