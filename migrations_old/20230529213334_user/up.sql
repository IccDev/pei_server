CREATE SCHEMA IF NOT EXISTS icc;

CREATE TABLE IF NOT EXISTS icc.users (
    email VARCHAR(100)  NOT NULL,
    id VARCHAR(200) NOT NULL,
    activated boolean,
    last_name VARCHAR(50),
    first_name VARCHAR(100),
    password TEXT NOT NULL,
    is_admin boolean,
    totp TEXT NOT NULL,
    two_factor boolean,
    created_at timestamptz,
    PRIMARY KEY(email)
);