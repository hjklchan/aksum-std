-- Add up migration script here
CREATE TABLE `users` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `username` varchar(255) NOT NULL,
  `avatar_url` varchar(255) DEFAULT NULL,
  `phone_number` varchar(11) DEFAULT NULL,
  `status` TINYINT NOT NULL DEFAULT 0,
  `password` varchar(255) DEFAULT NULL,
  `created_at` TIMESTAMP NULL DEFAULT NULL,
  `updated_at` TIMESTAMP NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
);