-- Your SQL goes here
CREATE TABLE `tags`
(
    `id`        numeric NOT NULL PRIMARY KEY,
    `name`      text    NOT NULL,
    `person_fk` numeric NULL,
    `entry_fk`  numeric NULL,


    FOREIGN KEY  (`person_fk`) REFERENCES `person` (`id`),
    FOREIGN KEY  (`entry_fk`) REFERENCES `entries` (`id`)
);
