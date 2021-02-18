-- Your SQL goes here
CREATE TABLE `simping`
(
    `person`   integer NOT NULL,
    `entry`    integer NOT NULL,
    `amount`   integer NOT NULL,
    `currency` text    NOT NULL,
    `is_worth` integer NOT NULL,

    PRIMARY KEY (`person`, `entry`),
    FOREIGN KEY  (`person`) REFERENCES `person` (`id`),
    FOREIGN KEY  (`entry`) REFERENCES `entries` (`id`)
);
