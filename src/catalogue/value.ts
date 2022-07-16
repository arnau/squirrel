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
  metadata: AssetMetadata;
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
