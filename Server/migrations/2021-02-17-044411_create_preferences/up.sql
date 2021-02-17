
CREATE TABLE `preferences`
(
    `id`                  numeric NOT NULL PRIMARY KEY,
    `autosave`            integer NULL DEFAULT 0,
    `autolock_interval`   integer NOT NULL DEFAULT 60,
    `backup`              integer NULL DEFAULT 0,
    `autodelete_interval` integer NULL,
    `online`              integer NOT NULL DEFAULT 0,
    `user`                numeric NOT NULL,
    `themes`         numeric NOT NULL,


    FOREIGN KEY  (`user`) REFERENCES `users` (`id`),
    FOREIGN KEY  (`themes`) REFERENCES `themes` (`id`)
);
