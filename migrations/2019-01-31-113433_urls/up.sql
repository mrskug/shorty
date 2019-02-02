-- Your SQL goes here
CREATE TABLE urls (
  id SERIAL PRIMARY KEY,
  hash VARCHAR NOT NULL,
  url VARCHAR NOT NULL
  )
