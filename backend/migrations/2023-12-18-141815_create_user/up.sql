-- Your SQL goes here
CREATE TABLE user (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  mail VARCHAR(150) NOT NULL,
  notifications BOOLEAN NOT NULL DEFAULT 1
)