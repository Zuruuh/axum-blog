CREATE TABLE IF NOT EXISTS app_users (
    id UUID PRIMARY KEY,
    email varchar(255) NOT NULL,
    email_canonical varchar(255) GENERATED ALWAYS AS (
        immutable_concat_ws('@', split_part(split_part(email, '@', 1), '+', 1), split_part(email, '@', -1))) STORED UNIQUE,
    password varchar(255) NOT NULL,
    email_verified bool NOT NULL DEFAULT FALSE
);
