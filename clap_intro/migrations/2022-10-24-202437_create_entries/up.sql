-- Your SQL goes here
CREATE TABLE entries (
    id SERIAL PRIMARY KEY,
    invoice INTEGER NOT NULL,
    name TEXT NOT NULL,
    paid BOOLEAN NOT NULL DEFAULT FALSE
)
