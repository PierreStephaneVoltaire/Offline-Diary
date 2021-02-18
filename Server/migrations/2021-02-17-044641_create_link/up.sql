-- Your SQL goes here
CREATE TABLE `links`
(
    `id`        integer  PRIMARY KEY  AUTOINCREMENT ,
    `link`      text    NOT NULL,
    `person_fk` integer NOT NULL,


    FOREIGN KEY  (`person_fk`) REFERENCES `person` (`id`)
);