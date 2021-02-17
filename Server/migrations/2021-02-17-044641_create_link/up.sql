-- Your SQL goes here
CREATE TABLE `links`
(
    `id`        numeric NOT NULL PRIMARY KEY,
    `link`      text    NOT NULL,
    `person_fk` numeric NOT NULL,


    FOREIGN KEY  (`person_fk`) REFERENCES `person` (`id`)
);