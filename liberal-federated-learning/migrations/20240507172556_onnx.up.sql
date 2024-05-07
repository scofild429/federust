-- Add up migration script here
CREATE TABLE onnxsets (
  id SERIAL PRIMARY KEY,
  onnxfile bytea
);
