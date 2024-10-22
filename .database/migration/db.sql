DROP TABLE IF EXISTS `tag`;
CREATE TABLE IF NOT EXISTS `tag` (
    `id` BIGINT AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,
    `deleted_at` TIMESTAMP,
    PRIMARY KEY(`id`)
);
DROP TABLE IF EXISTS `category`;
CREATE TABLE IF NOT EXISTS `tag` (
    `id` BIGINT AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    `parent_id` BIGINT NOT NULL DEFAULT 0,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,
    `deleted_at` TIMESTAMP,
    PRIMARY KEY(`id`),
    INDEX `idx_parent` (`parent_id`),
    UNIQUE (`name`)
);
DROP TABLE IF EXISTS `data`;
CREATE TABLE IF NOT EXISTS `data` (
    `id` BIGINT AUTO_INCREMENT,
    `category_id` BIGINT NOT NULL,
    `sub_category_id` BIGINT NOT NULL,
    `tag_id` BIGINT NOT NULL,
    `amount` DOUBLE NOT NULL,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,
    `deleted_at` TIMESTAMP,
    PRIMARY KEY(`id`),
    UNIQUE INDEX `idx_unique` (`tag_id`, `category_id`, `sub_category_id`),
);