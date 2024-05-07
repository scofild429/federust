-- Add up migration script here
DROP TABLE IF EXISTS "member";

CREATE TABLE "member" (
  id integer NOT NULL PRIMARY KEY,
  email VARCHAR(255) NOT NULL UNIQUE,
  password varchar(100) NOT NULL,
  username varchar(32) NOT NULL,
  joined_date date NOT NULL,
  created_date timestamp with time zone NOT NULL
);

