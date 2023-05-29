-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    second_name VARCHAR(255) NOT NULL
);

CREATE TABLE charts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    user_id SERIAL REFERENCES users(id)
);

CREATE TABLE navigations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    url VARCHAR(255) NOT NULL
);

CREATE TABLE user_navigations (
    id SERIAL PRIMARY KEY,
    user_id SERIAL REFERENCES users(id),
    navigation_id SERIAL REFERENCES navigations(id),
    active BOOLEAN NOT NULL DEFAULT FALSE
);
