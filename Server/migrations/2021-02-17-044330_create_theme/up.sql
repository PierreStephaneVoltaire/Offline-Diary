-- Your SQL goes here
-- Your SQL goes here

CREATE TABLE `themes`
(
    `id`           integer   PRIMARY KEY AUTOINCREMENT,
    `name`        text    NOT NULL DEFAULT "",
    `main_color`  text    NOT NULL DEFAULT "green",
    `font_family` text    NOT NULL DEFAULT "fira",
    `serif`       integer NOT NULL DEFAULT 1

);