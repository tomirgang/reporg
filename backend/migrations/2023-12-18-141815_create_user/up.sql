-- Your SQL goes here
CREATE TABLE user (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name VARCHAR(150) NOT NULL,
  email VARCHAR(150) NOT NULL UNIQUE,
  phone VARCHAR(150) NOT NULL,
  notifications BOOLEAN NOT NULL DEFAULT 1,
  roles INTEGER NOT NULL DEFAULT 0
)