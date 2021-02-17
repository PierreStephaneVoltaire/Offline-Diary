-- Your SQL goes here
CREATE TABLE `nicknames`
(
    `id`        numeric NOT NULL PRIMARY KEY,
    `nickname`  text    NOT NULL,
    `person_fk` numeric NOT NULL,
    FOREIGN KEY  (`person_fk`) REFERENCES `person` (`id`)
);