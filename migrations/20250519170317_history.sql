CREATE TABLE history(
  deck NUMERIC NOT NULL,
  offset INTEGER NOT NULL,
  time DATETIME NOT NULL,
  PRIMARY KEY (deck, offset)
);
