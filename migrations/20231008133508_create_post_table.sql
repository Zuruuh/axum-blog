-- Add migration script here
CREATE TABLE IF NOT EXISTS app_posts (
    id UUID PRIMARY KEY,
    title varchar(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    last_update TIMESTAMPTZ NOT NULL,
    UNIQUE (title)
)
