-- Your SQL goes here

CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

CREATE TABLE accounts (
    account_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    value REAL,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(user_id)
);

CREATE TABLE transactions (
    transaction_id SERIAL PRIMARY KEY,
    account_id INT NOT NULL,
    description TEXT,
    before REAL,
    after REAL,
    amount REAL,
    meta json,
    CONSTRAINT fk_account FOREIGN KEY(account_id) REFERENCES accounts(account_id)
);

CREATE INDEX account_id on transactions (account_id);
