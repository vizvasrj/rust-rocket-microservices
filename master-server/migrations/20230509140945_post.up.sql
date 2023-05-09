-- Add up migration script here
CREATE TABLE IF NOT EXISTS posts
(
    uuid UUID PRIMARY KEY,
    user_uuid UUID,
    CONSTRAINT fk_user
        FOREIGN KEY(user_uuid)
            REFERENCES "users"(uuid)
            ON DELETE CASCADE,
    title VARCHAR NOT NULL,
    body TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()

)