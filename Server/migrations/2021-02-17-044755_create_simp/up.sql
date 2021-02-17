-- Your SQL goes here
CREATE TABLE `simping`
(
    `person`   numeric NOT NULL,
    `entry`    numeric NOT NULL,
    `amount`   numeric NOT NULL,
    `currency` text    NOT NULL,
    `is_worth` numeric NOT NULL,

    PRIMARY KEY (`person`, `entry`),
    FOREIGN KEY  (`person`) REFERENCES `person` (`id`),
    FOREIGN KEY  (`entry`) REFERENCES `entries` (`id`)
);
