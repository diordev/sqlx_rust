-- Add migration script here
ALTER TABLE users
ADD COLUMN phone TEXT UNIQUE;
