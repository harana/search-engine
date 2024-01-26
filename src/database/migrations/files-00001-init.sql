CREATE TABLE files (
  path TEXT NOT NULL,
  hash TEXT,
  index_date BIGINT,
  thumbnail_date BIGINT,
  document_id TEXT UNIQUE,
  PRIMARY KEY (path)
);

CREATE INDEX document_id_null ON files(path);
CREATE INDEX index_date_null ON files(path) WHERE index_date IS NULL;
CREATE INDEX thumbnail_date_null ON files(path) WHERE thumbnail_date IS NULL;