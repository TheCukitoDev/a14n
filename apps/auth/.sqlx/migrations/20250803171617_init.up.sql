-- Add up migration script here
CREATE TABLE IF NOT EXISTS `users` (
     `user_id` INT NOT NULL, 
     `username` VARCHAR(50) NOT NULL,
     PRIMARY KEY (`user_id`)
 );