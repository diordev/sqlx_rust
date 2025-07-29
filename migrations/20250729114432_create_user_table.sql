-- Add migration: create_users

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

insert into users (username, email) values
('test1', 'test1@mail.com'),
('test2', 'test2@mail.com'),
('test3', 'test3@mail.com'); -- Note: This will fail due to unique constraint on email
