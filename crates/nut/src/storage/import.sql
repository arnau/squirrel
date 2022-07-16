CREATE TEMPORARY VIEW import_root AS
SELECT
    id_global AS id,
    name,
    absolutePath AS path
FROM
    catalogue.AgLibraryRootFolder
WHERE
    relativePathFromCatalog IS NOT NULL;


CREATE TEMPORARY VIEW import_folder AS
SELECT
    folder.id_global AS id,
    '/' || root_folder.name || '/' || folder.pathFromRoot AS path,
    'folder' AS kind,
    (
        CASE parent_path('/' || root_folder.name || '/' || folder.pathFromRoot)
        WHEN NULL THEN
            NULL
        WHEN '/' || root_folder.name || '/' THEN
            (
                SELECT
                    f.id_global
                FROM
                    AgLibraryFolder AS f
                WHERE
                    f.pathFromRoot = ''
                AND
                    f.rootFolder = root_folder.id_local
                LIMIT 1
            )
        ELSE
            (
                SELECT
                    f.id_global
                FROM
                    AgLibraryFolder AS f
                WHERE
                    f.pathFromRoot = parent_path(folder.pathFromRoot)
                LIMIT 1
            )
        END
    ) AS parent_id,
    root_folder.id_global AS root_id
FROM
    catalogue.AgLibraryFolder AS folder
INNER JOIN
    catalogue.AgLibraryRootFolder AS root_folder
    ON folder.rootFolder = root_folder.id_local
WHERE
    root_folder.relativePathFromCatalog IS NOT NULL;


CREATE TEMPORARY VIEW import_file AS
SELECT
    file.id_global AS id,
    '/' || root_folder.name || '/' || folder.pathFromRoot || file.idx_filename AS path,
    'file' AS kind,
    folder.id_global AS parent_id,
    root_folder.id_global AS root_id
FROM
    catalogue.AgLibraryFile AS file
INNER JOIN
    catalogue.AgLibraryFolder AS folder
    ON file.folder = folder.id_local
INNER JOIN
    catalogue.AgLibraryRootFolder AS root_folder
    ON root_folder.id_local = folder.rootFolder
WHERE
    root_folder.relativePathFromCatalog IS NOT NULL;


CREATE TEMPORARY VIEW import_asset AS
SELECT
    image.id_global AS id,
    file.id_global AS entry_id,
    coalesce(image.rating, 0) AS rating,
    image.pick AS flag,
    iif(image.colorLabels = '', NULL, image.colorLAbels) AS label,
    image.fileFormat AS format,
    image.fileWidth AS width,
    image.fileHeight AS height,
    image.orientation AS orientation,
    (
        select i.id_global from catalogue.Adobe_images AS i where image.masterImage = i.id_local
    ) AS master_id,
    cache.uuid AS pyramid_uuid,
    cache.digest AS pyramid_digest,
    strftime('%Y-%m-%dT%H:%M:%SZ',
        iif(file.modTime IS NULL, 63113817600, file.modTime + 978307200),
        'unixepoch') AS modification_stamp
FROM
    catalogue.Adobe_images AS image
INNER JOIN
    catalogue.AgLibraryFile AS file
    ON file.id_local = image.rootFile
INNER JOIN
    catalogue.AgLibraryFolder AS folder
    ON file.folder = folder.id_local
INNER JOIN
    catalogue.AgLibraryRootFolder AS root_folder
    ON folder.rootFolder = root_folder.id_local
INNER JOIN
    previews.ImageCacheEntry AS cache
    ON cache.imageId = image.id_local
WHERE
    root_folder.relativePathFromCatalog IS NOT NULL;
