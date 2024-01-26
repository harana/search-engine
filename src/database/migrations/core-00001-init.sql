CREATE TABLE adjusted_terms (
  term TEXT NOT NULL,
  language TEXT NOT NULL,
  category INT NOT NULL,
  direction INT NOT NULL,
  user_added INT,
  PRIMARY KEY (term, language)
);

CREATE INDEX term_language_direction ON adjusted_terms(category, term, direction);
CREATE INDEX user_added ON adjusted_terms(user_added) WHERE user_added IS NOT NULL;

--------------------------------------------------------------------------------------

CREATE TABLE ai_models (
  id TEXT NOT NULL,
  status TEXT NOT NULL,
  enabled INT NOT NULL,
  indexer_type TEXT NOT NULL,
  name TEXT NOT NULL,
  version TEXT NOT NULL,
  creation_date BIGINT NOT NULL,
  PRIMARY KEY (id)
);

--------------------------------------------------------------------------------------

CREATE TABLE developer_sources (
  title TEXT NOT NULL,
  name TEXT NOT NULL,
  language TEXT NOT NULL,
  enabled INT,
  PRIMARY KEY (name)
);

--------------------------------------------------------------------------------------

CREATE TABLE job_groups (
  category TEXT NOT NULL,
  name TEXT NOT NULL,
  successes INT NOT NULL,
  failures INT NOT NULL,
  total INT NOT NULL,
  PRIMARY KEY (category, name)
);

--------------------------------------------------------------------------------------

CREATE TABLE private_folders (
  path TEXT NOT NULL,
  PRIMARY KEY (path)
);

--------------------------------------------------------------------------------------

CREATE TABLE recent_documents (
  search_query TEXT NOT NULL,
  documents TEXT,
  PRIMARY KEY (search_query)
);

--------------------------------------------------------------------------------------

CREATE TABLE rules (
  id TEXT NOT NULL,
  source_type TEXT NOT NULL,
  qualifier_type TEXT NOT NULL,
  qualifier_value TEXT NOT NULL,
  action_type TEXT NOT NULL,
  action_value TEXT NOT NULL,
  PRIMARY KEY (id)
);

--------------------------------------------------------------------------------------

CREATE TABLE search_categories (
  title TEXT NOT NULL,
  name TEXT NOT NULL,
  enabled INT,
  position INT,
  PRIMARY KEY (name)
);

--------------------------------------------------------------------------------------

CREATE TABLE search_folders (
  title TEXT NOT NULL,
  name_or_path TEXT NOT NULL,
  is_path INT,
  enabled INT,
  crawled INT,
  PRIMARY KEY (name_or_path)
);

--------------------------------------------------------------------------------------

CREATE TABLE settings (
  key TEXT NOT NULL,
  value TEXT,
  PRIMARY KEY (key)
);

--------------------------------------------------------------------------------------

CREATE TABLE state (
  key TEXT NOT NULL,
  value TEXT,
  PRIMARY KEY (key)
);