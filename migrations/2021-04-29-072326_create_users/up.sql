-- Your SQL goes here
CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  user_uuid uuid NOT NULL DEFAULT uuid_generate_v1(),
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  hash VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);