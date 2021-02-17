-- Your SQL goes here
CREATE TABLE `mentions`
(
    `entry`  numeric NOT NULL,
    `person` numeric NOT NULL,

    PRIMARY KEY (`entry`, `person`),
    FOREIGN KEY  (`entry`) REFERENCES `entries` (`id`),
    FOREIGN KEY  (`person`) REFERENCES `person` (`id`)
);