
CREATE TABLE `preferences`
(
    `id`                   integer PRIMARY KEY  AUTOINCREMENT ,
    `autosave`            integer NULL DEFAULT 0,
    `autolock_interval`   integer NOT NULL DEFAULT 60,
    `backup`              integer NULL DEFAULT 0,
    `autodelete_interval` integer NULL,
    `online`              integer NOT NULL DEFAULT 0,
    `user`                integer NOT NULL,
    `themes`         integer NOT NULL,


    FOREIGN KEY  (`user`) REFERENCES `users` (`id`),
    FOREIGN KEY  (`themes`) REFERENCES `themes` (`id`)
);
