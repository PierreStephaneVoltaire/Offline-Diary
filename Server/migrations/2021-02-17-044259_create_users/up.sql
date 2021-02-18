-- Your SQL goes here
-- Your SQL goes here


CREATE TABLE `users`
(
    `id`       integer    PRIMARY KEY AUTOINCREMENT,
    `name`     text    NOT NULL,
    `gdrive`   TEXT        NOT NULL,
    `password` integer NOT NULL,
    FOREIGN KEY  (`password`) REFERENCES `password` (`id`)
);





