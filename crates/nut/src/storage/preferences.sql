/* This SQL script defines the schema for the user preferences.
 */


-- Metadata about the Squirrel catalogue.
CREATE TABLE IF NOT EXISTS squirrel (
    key   text NOT NULL PRIMARY KEY,
    value text NOT NULL
);

CREATE TABLE IF NOT EXISTS preferences (
    key   text NOT NULL PRIMARY KEY,
    value text NOT NULL
);

-- Connectors for BackBlaze.
CREATE TABLE IF NOT EXISTS connector (
    id             text      NOT NULL PRIMARY KEY, -- backblaze key id
    key_name       text      NOT NULL, -- backblaze key name
    bucket_name    text      NOT NULL, -- backblaze bucket name
    secret_key     boolean   NOT NULL, -- the actual value is stored in the SO keyring
    kind           text      NOT NULL, -- always 'backblaze'
    creation_stamp timestamp NOT NULL
);


-- Connectors used by each source, potentially per root.
CREATE TABLE IF NOT EXISTS source_connector (
    source_id    text NOT NULL,
    connector_id text NOT NULL,
    root_id      text, -- when NULL, the connector is the default for all roots
    base_path    text NOT NULL, -- the connector base path which combined with an file path gives the bb file name

    FOREIGN KEY (source_id) REFERENCES source (id),
    FOREIGN KEY (connector_id) REFERENCES connector (id),
    FOREIGN KEY (root_id) REFERENCES root (id)
);

CREATE INDEX IF NOT EXISTS idx_source_connector_source_id ON source_connector (source_id);

-- source events
CREATE TABLE IF NOT EXISTS source_event (
    stamp     timestamp NOT NULL,
    source_id text NOT NULL,
    action    text NOT NULL, -- import, refresh, remove
    status    text NOT NULL, -- success, fail
    summary   text,          -- textual summary e.g. when a failure happens

    FOREIGN KEY (source_id) REFERENCES source (id)
);
