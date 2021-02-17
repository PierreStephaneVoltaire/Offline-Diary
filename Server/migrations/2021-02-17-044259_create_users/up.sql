-- Your SQL goes here
-- Your SQL goes here


CREATE TABLE `users`
(
    `id`       numeric NOT NULL PRIMARY KEY,
    `name`     text    NOT NULL,
    `gdrive`   TEXT        NOT NULL,
    `password` numeric NOT NULL,
    FOREIGN KEY  (`password`) REFERENCES `password` (`id`)
);





