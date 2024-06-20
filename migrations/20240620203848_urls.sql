-- Add migration script here
CREATE TABLE urls (
  id VARCHAR(255) PRIMARY KEY,
  url TEXT NOT NULL
);