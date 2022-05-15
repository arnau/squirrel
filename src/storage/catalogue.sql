/* This SQL script defines the schema for the catalogue.
 *
 * Its main purpose is to keep data about files, folders, thumbnails, metadata.
 */


-- Metadata about the Squirrel catalogue.
CREATE TABLE IF NOT EXISTS catalogue_metadata (
  key   text NOT NULL,
  value text NOT NULL
);

-- A source to gather data from.
CREATE TABLE IF NOT EXISTS source (
  id                text NOT NULL PRIMARY KEY,
  -- e.g. '2021_JC_Candanedo-v11'
  name              text NOT NULL,
  -- e.g. '/absolute/path/to/catalogue'
  path              text NOT NULL,
  -- e.g. 1100000
  version           integer NOT NULL
);

-- AgLibraryRootFolder copy
CREATE TABLE IF NOT EXISTS root (
  -- id_global
  id        text NOT NULL PRIMARY KEY,
  name      text NOT NULL,
  -- filesystem root path
  path      text NOT NULL,
  source_id text NOT NULL,

  UNIQUE (path, source_id),
  FOREIGN KEY (source_id) REFERENCES source (id)
);

-- A file system entry. Either a folder or a file.
CREATE TABLE IF NOT EXISTS entry (
  -- id_global
  id        text NOT NULL PRIMARY KEY,
  -- the last stem from the path.
  -- name      text NOT NULL,
  path      text NOT NULL,
  kind      text NOT NULL, -- (file, folder)
  parent_id text,
  root_id   text NOT NULL,
  source_id text NOT NULL,

  -- review uniqueness
  UNIQUE (path, source_id),
  FOREIGN KEY (parent_id) REFERENCES entry (id),
  FOREIGN KEY (root_id)   REFERENCES root (id),
  FOREIGN KEY (source_id) REFERENCES source (id)
);


-- Any entry that is a file and is an image
CREATE TABLE IF NOT EXISTS asset (
  id                text NOT NULL PRIMARY KEY,
  -- from Adobe_images.rating
  rating            number,
  -- from Adobe_images.pick
  flag              boolean,
  -- from Adobe_images.colorLabel
  label             text,
  -- TIFF, JPEG, etc
  format            text NOT NULL,
  width             number NOT NULL,
  height            number NOT NULL,
  orientation       text NOT NULL,

  pyramid_uuid      text NOT NULL,
  pyramid_digest    text NOT NULL,
  pyramid_filename  text AS (pyramid_uuid || '-' || pyramid_digest || '.lrprev'),

  modification_time timestamp NOT NULL,

  entry_id          text NOT NULL,

  FOREIGN KEY (entry_id) REFERENCES entry (id)
);

-- event log
CREATE TABLE IF NOT EXISTS event (
  data   text NOT NULL,
  stamp  timestamp AS (json_extract(data, '$.stamp')),
  squirrel_version text AS (json_extract(data, '$.squirrel_version')),
  action text AS (json_extract(data, '$.action'))
);
