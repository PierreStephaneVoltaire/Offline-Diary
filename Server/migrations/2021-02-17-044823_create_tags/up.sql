-- Your SQL goes here
CREATE TABLE `tags`
(
    `id`        integer  PRIMARY KEY  AUTOINCREMENT ,
    `name`      text    NOT NULL,
    `person_fk` integer NULL,
    `entry_fk`  integer NULL,


    FOREIGN KEY  (`person_fk`) REFERENCES `person` (`id`),
    FOREIGN KEY  (`entry_fk`) REFERENCES `entries` (`id`)
);
