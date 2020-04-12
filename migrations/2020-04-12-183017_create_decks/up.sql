-- Your SQL goes here

CREATE TABLE decks(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    tags TEXT [] NOT NULL,
    cards jsonb NOT NULL
)