-- Add migration script here
CREATE TYPE chocolate_type AS ENUM ('Bitter', 'White', 'Milk');

CREATE TABLE IF NOT EXISTS products (
    id serial primary key,
    name text,
    price integer,
    chocolate_type chocolate_type,
    fillings text[],
    images text[]
);