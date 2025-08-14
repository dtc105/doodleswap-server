CREATE SCHEMA IF NOT EXISTS doodleswap;

CREATE TABLE IF NOT EXISTS doodleswap.user (
    id SERIAL PRIMARY KEY,
    email VARCHAR(254) NOT NULL,
    username VARCHAR(32) NOT NULL,
    password_hash CHAR(60) NOT NULL,
    pfp_path VARCHAR(255),
    pfp_mime_type VARCHAR(50),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    role VARCHAR(5) NOT NULL
);

CREATE TABLE IF NOT EXISTS doodleswap.character (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    first_name VARCHAR(32) NOT NULL,
    last_name VARCHAR(32),
    pronouns VARCHAR(32),
    quote VARCHAR(100),
    image_path VARCHAR(255) NOT NULL,
    image_mime_type VARCHAR(50) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES doodleswap.user(id) ON DELETE CASCADE
);