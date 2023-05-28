/** An identifier for any location */
export type LocationId = GroundId | FolderId | AssetId


/** An identifier for the ground */
export type GroundId = ""

/** An identifier for a folder */
export type FolderId = string

/** A path for a folder. Not to confuse with a `Route` */
export type FolderPath = string

/** A trail from root to leaf for a location. */
export type Trail = Array<FolderId>

export interface GroundLocation {
  kind: "Ground",
  id: GroundId,
}

export interface FolderLocation {
  kind: "Folder",
  id: FolderId,
  path: FolderPath,
  trail: Trail,
}

export interface AssetLocation {
  kind: "Asset",
  id: AssetId,
  path: AssetPath,
  trail: Trail,
}

/** A catalogue entry location. */
export type Location = GroundLocation | FolderLocation | AssetLocation


/** A tree item */
export interface FolderEntry {
  id: FolderId,
  path: FolderPath,
  counter: number,
}

/** The list of available root folders */
export type Ground = Array<FolderEntry>

/** The list of folders for a `Location` */
export type LocationFolders = Array<FolderEntry>


/** An asset unique identifier */
export type AssetId = string;

/** An asset path.
  *
  * See Rust's `nut::entities::state::AssetPath`.
  */
export type AssetPath = string;


/** A cursor for paging assets.
  *
  * See Rust's `nut::entities::state::AssetCursor`
  */
export type AssetCursor = string | null

/** An asset page for a `Location` */
export interface LocationAssetPage {
  next_cursor: AssetCursor,
  data: Array<Asset>,
}

/** The list of assets for a given location */
export type LocationAssets = Array<Asset>

/** The local consolidated asset store.
  *
  * - When `cursor` is `undefined`, the list of assets is `undefined`. Either
  *   the first page has not been fetched yet or the current location has no
  *   assets.
  * - When `cursor` is `null` it is assumed the list of assets is complete.
  */
export interface AssetStore {
  cursor?: AssetCursor,
  parentId?: FolderId,
  assets?: LocationAssets,
}

/** An asset for the current `Location` */
export interface Asset {
  id: AssetId;
  path: AssetPath;
  master_id: AssetId | null;
  metadata: AssetMetadata;
}

export interface AssetMetadata {
  rating: number | null;
  flag: boolean;
  label: string | null;
  format: string;
  width: number;
  height: number;
  orientation: string;
}

/** A thumbnail blob as base64 */
export type Thumbnail = string

export interface ThumbnailStore {
  [id: AssetId]: Thumbnail,
}


/** The UI cache for the tree nodes that have been loaded */
export interface FolderMap {
  [id: FolderId]: LocationFolders,
}

/** The UI state tracking what is visible */
export interface State {
  isDetailsOpen: boolean,
  tree: TreeState,
  isImageFullsize: boolean,
}

/** The folder tree state.
  *
  * It complements `FolderMap` which holds the actual folder data.
  */
export interface TreeState {
  [id: FolderId]: TreeItemState,
}

export interface TreeItemState {
  isOpen: boolean,
}


/** A metadata set for a folder (see `AssetMetadata` for the other entry type) */
export interface FolderDetails {
  id: FolderId,
  path: FolderPath,
  source: Source,
  root: Root,
  folder_count: number,
  asset_count: number,
}

export interface Root {
  id: string,
  name: string,
  path: string,
}

export interface Source {
  id: string,
  name: string,
  path: string,
  version: number,
}


// TODO: start review


export interface Image {
  blob: Blob;
  width: number;
  height: number;
}

export interface Blob {
  width: number;
  height: number;
  data: Uint8Array;
}


export interface Reference {
  kind: "Path" | "OriginalId";
  value: string;
}

// end review


