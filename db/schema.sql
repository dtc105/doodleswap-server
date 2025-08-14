SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

CREATE SCHEMA doodleswap;

CREATE TABLE doodleswap."user" (
    id SERIAL PRIMARY KEY,
    email VARCHAR(254) NOT NULL,
    username VARCHAR(32) NOT NULL,
    password_hash CHAR(60) NOT NULL,
    pfp_path VARCHAR(255),
    pfp_mime_type VARCHAR(50),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    role VARCHAR(5) NOT NULL
);
