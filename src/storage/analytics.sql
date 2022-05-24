/**
 * Counts the number of selected assets per folder.
 *
 * A selected asset has either a rating, a flag or a label.
 */
CREATE VIEW IF NOT EXISTS entry_selected_assets AS
SELECT
    parent.id AS id,
    parent.path AS path,
    count(entry.path) AS number
FROM
    entry
INNER JOIN
    asset
    ON asset.entry_id = entry.id
INNER JOIN
    entry AS parent
    ON parent.id = entry.parent_id
WHERE
    asset.rating > 0
OR
    asset.flag <> 0
OR
    asset.label IS NOT NULL
GROUP BY
    entry.parent_id;

/**
 * Counts the number of assets per folder.
 *
 * See `entry_selected_assets` for a filtered alternative.
 */
CREATE VIEW IF NOT EXISTS entry_assets AS
SELECT
    parent.id AS id,
    parent.path AS path,
    count(entry.path) AS number
FROM
    entry
INNER JOIN
    asset
    ON asset.entry_id = entry.id
INNER JOIN
    entry AS parent
    ON parent.id = entry.parent_id
GROUP BY
    entry.parent_id;


/**
 * Counts the number of subfolders per folder.
 */
CREATE VIEW IF NOT EXISTS entry_subfolders AS
SELECT
    parent.id AS id,
    parent.path AS path,
    count(entry.path) AS number
FROM
    entry
INNER JOIN
    entry AS parent
    ON parent.id = entry.parent_id
WHERE
    entry.kind = 'folder'
GROUP BY
    entry.parent_id;
