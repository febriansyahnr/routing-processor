-- Add up migration script here
CREATE TABLE IF NOT EXISTS `transfer_configs` (
    `uuid` varchar(36) NOT NULL,
    `processor_uuid` varchar(36) NOT NULL,
    `priority` int NOT NULL,
    `module` varchar(50) NOT NULL DEFAULT 'Default',
    `method` varchar(50) NOT NULL DEFAULT '' COMMENT 'INTRABANK,INTERBANK,BIFAST,etc',
    `limit_config` json DEFAULT NULL,
    `status` varchar(15) NOT NULL DEFAULT 'active',
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`uuid`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;