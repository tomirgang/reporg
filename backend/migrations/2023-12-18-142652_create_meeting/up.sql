-- Your SQL goes here
CREATE TABLE meeting (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  time DATETIME NOT NULL,
  confirmed BOOLEAN NOT NULL DEFAULT 0,
  cafe INTEGER NOT NULL,
  device INTEGER NOT NULL,
  supporter INTEGER NOT NULL,
  FOREIGN KEY (cafe) REFERENCES cafe(id) ON DELETE CASCADE,
  FOREIGN KEY (device) REFERENCES device(id) ON DELETE CASCADE,
  FOREIGN KEY (supporter) REFERENCES user(id) ON DELETE CASCADE
)