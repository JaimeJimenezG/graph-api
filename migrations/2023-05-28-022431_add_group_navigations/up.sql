-- Your SQL goes here
CREATE TABLE IF NOT EXISTS group_navigations (
    id SERIAL PRIMARY KEY,
    group_id SERIAL REFERENCES groups(id),
    navigation_id SERIAL REFERENCES navigations(id),
    active BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);