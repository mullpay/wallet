-- Add up migration script here

CREATE TABLE account (
    id BLOB PRIMARY KEY NOT NULL,
    public_key BLOB NOT NULL UNIQUE,
    eulen_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
) STRICT;