-- Your SQL goes here
-- Your SQL goes here

CREATE TABLE `password`
(
    `id`           integer  PRIMARY KEY  AUTOINCREMENT  ,
    `passwordval`     text    NOT NULL,
    `q1`           text    NOT NULL,
    `q2`           text    NOT NULL,
    `q3`           text    NOT NULL,
    `a1`           text    NOT NULL,
    `a2`           text    NOT NULL,
    `a3`           text    NOT NULL,
    `wipe_attempt` integer NOT NULL DEFAULT 3
);




