-- Your SQL goes here
ALTER TABLE users ADD password varchar(255) NOT NULL DEFAULT 'admin';
ALTER TABLE users ADD nickname varchar(255) NOT NULL DEFAULT 'admin';
