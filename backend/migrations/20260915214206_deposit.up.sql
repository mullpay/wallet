-- Add up migration script here

CREATE TABLE deposit (
    id BLOB PRIMARY KEY NOT NULL,
    account_id BLOB NOT NULL,
    deposit_id BLOB NOT NULL UNIQUE,
    status INTEGER NOT NULL,
    deposit_amount_in_cents INTEGER NOT NULL,
    payout_amount_in_cents INTEGER NOT NULL,
    fee_amount_in_cents INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES account (id)
) STRICT;