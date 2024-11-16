-- Add up migration script here
CREATE TABLE IF NOT EXISTS processors
(
    uuid VARCHAR(36) PRIMARY KEY NOT NULL,
    name VARCHAR(50) NOT NULL,
    description TEXT NULL,
    base_url VARCHAR(255) NOT NULL,
    status VARCHAR(10) NOT NULL DEFAULT 'ACTIVE',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);