CREATE DATABASE trend;

CREATE TABLE trend_info (
    id UUID PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    link VARCHAR(255) NOT NULL,
    title VARCHAR(255),
    desc TEXT,
    memo TEXT,
    from VARCHAR(255),
    status VARCHAR(255),
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    UNIQUE (user_id, link)
);
