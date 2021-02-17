-- Your SQL goes here
-- Your SQL goes here

CREATE TABLE `password`
(
    `id`           numeric NOT NULL ,
    `passwordval`     text    NOT NULL,
    `q1`           text    NOT NULL,
    `q2`           text    NOT NULL,
    `q3`           text    NOT NULL,
    `a1`           text    NOT NULL,
    `a2`           text    NOT NULL,
    `a3`           text    NOT NULL,
    `wipe_attempt` numeric NOT NULL DEFAULT 3,
    PRIMARY KEY (`id`)
);




