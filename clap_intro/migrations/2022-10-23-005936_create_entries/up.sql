-- Your SQL goes here
CREATE TABLE entries (
    id SERIAL PRIMARY KEY,
    invoice INTEGER,
    name VARCHAR ,
    paid BOOLEAN DEFAULT FALSE
)
