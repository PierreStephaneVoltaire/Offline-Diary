-- Your SQL goes here
CREATE TABLE `nicknames`
(
    `id`        integer  PRIMARY KEY  AUTOINCREMENT ,
    `nickname`  text    NOT NULL,
    `person_fk` integer NOT NULL,
    FOREIGN KEY  (`person_fk`) REFERENCES `person` (`id`)
);