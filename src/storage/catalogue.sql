/* This SQL script defines the schema for the catalogue.
 *
 * Its main purpose is to keep data about files, folders, thumbnails, metadata.
 */


-- Metadata about the catalogue. TODO: Should this be in para?
CREATE TABLE IF NOT EXISTS catalogue_metadata (
  version text NOT NULL,
  insert_stamp timestamp NOT NULL
);

-- A source to gather data from.
CREATE TABLE IF NOT EXISTS source (
  id                text NOT NULL PRIMARY KEY,
  -- e.g. '2021_JC_Candanedo-v11'
  name              text NOT NULL,
  -- e.g. '/absolute/path/to/catalogue'
  path              text NOT NULL,
  -- e.g. '1100000'
  version           text NOT NULL,

  -- para
  insert_stamp timestamp NOT NULL,
  update_stamp timestamp NOT NULL,
);

-- AgLibraryRootFolder copy
CREATE TABLE IF NOT EXISTS root (
  id        text NOT NULL PRIMARY KEY,
  name      text NOT NULL,
  -- filesystem root path
  path      text NOT NULL,
  source_id text NOT NULL,

  -- para
  insert_stamp timestamp NOT NULL,
  update_stamp timestamp NOT NULL,

  UNIQUE (path, source_id),
  FOREIGN KEY (source_id) REFERENCES source (id)
);

-- A file system entry. Either a folder or a file.
CREATE TABLE IF NOT EXISTS entry (
  -- TODO: digest of the path
  id        text NOT NULL PRIMARY KEY,
  -- path starting from the source root.
  path      text NOT NULL,
  -- the last stem from the path.
  name      text NOT NULL,
  parent_id text,
  kind      text, -- (file, folder)
  source_id text NOT NULL,
  root_id   text NOT NULL,

  -- para
  insert_stamp timestamp NOT NULL,
  update_stamp timestamp NOT NULL,

  -- review uniqueness
  UNIQUE (path, source_id),
  FOREIGN KEY (parent_id) REFERENCES entry (id),
  FOREIGN KEY (source_id) REFERENCES source (id),
  FOREIGN KEY (root_id) REFERENCES root (id)
);


-- Any entry that is a file and is an image
CREATE TABLE IF NOT EXISTS asset (
  id                text NOT NULL PRIMARY KEY,
  entry_id          text NOT NULL,
  -- from Adobe_images.rating
  rating            number,
  -- from Adobe_images.colorLabel
  label             text,
  -- TIFF, JPEG, etc
  format            text NOT NULL,
  width             number NOT NULL,
  height            number NOT NULL,
  orientation       text NOT NULL,
  -- from previews.ImageCacheEntry.(uuid || '-' || digest)
  /* pyramid_id text NOT NULL */
  pyramid_uuid      text NOT NULL,
  pyramid_digest    text NOT NULL,

  modification_time timestamp NOT NULL
);

