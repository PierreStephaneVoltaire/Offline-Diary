-- Your SQL goes here
CREATE TABLE `mentions`
(
    `entry`  integer NOT NULL,
    `person` integer NOT NULL,

    PRIMARY KEY (`entry`, `person`),
    FOREIGN KEY  (`entry`) REFERENCES `entries` (`id`),
    FOREIGN KEY  (`person`) REFERENCES `person` (`id`)
);