import { Route } from "../aux/route";

/** Represents a catalogue entry.
  *
  * Location: The ordered set of stems that compose the route.
  * Roots: The set of known root folders. This is fixed for any request and only changes when a new LR catalogue is imported.
  * Folders: The set of folders siblings to the current folder. TODO: Review this logic.
  * Assets: The set of assets contained in the current folder.
  */
export interface Value {
  kind: "Catalogue";
  location: Location;
  roots: Array<Folder>;
  folders: Array<Folder>;
  assets: Array<Asset>;
}

export type AssetId = string;

export interface Image {
  blob: Blob;
  width: number;
  height: number;
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

export interface Blob {
  width: number;
  height: number;
  data: Uint8Array;
}

export interface Asset {
  kind: "Asset";
  id: AssetId;
  path: Route;
  master_id: AssetId | null;
  metadata: AssetMetadata;
}

export interface Reference {
  kind: "Path" | "OriginalId";
  value: string;
}

export interface Folder {
  kind: "Folder";
  id: string;
  path: Route;
}

export type Stem = Folder | Asset

export interface Location {
  path: Route;
  stems: Array<Stem>;
}

/**
  * Maps to Rust's `State::Ground`
  */
export interface StateGround {
  kind: "Ground",
  roots: Array<Folder>,
}


/**
  * Maps to Rust's `State::Tree`
  */
export interface StateTree {
  kind: "Tree",
  path: Route,
  value: Tree,
}

export type Tree = TreeLeaf | TreeNode | TreeEmpty

export interface TreeEmpty {
  kind: "Empty",
}

export interface TreeLeaf {
  kind: "Leaf",
  path: Route,
}

export interface TreeNode {
  kind: "Node",
  path: Route,
  children: Array<Tree>,
}

export interface State {
  // folder details
  isDetailsOpen: boolean,
  tree: TreeState,
}

export interface TreeState {
  [route: Route]: TreeItemState,
}

export interface TreeItemState {
  isOpen: boolean,
}
