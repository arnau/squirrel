/** Represents a catalogue entry.
  *
  * Location: The ordered set of stems that compose the route.
  * Roots: The set of known root folders. This is fixed for any request and only changes when a new LR catalogue is imported.
  * Folders: The set of folders siblings to the current folder. TODO: Review this logic.
  * Files: The set of files contained in the current folder.
  */
export interface Value {
  kind: "Catalogue";
  location: Location;
  roots: Array<Folder>;
  folders: Array<Folder>;
  files: Array<File>;
}

export interface Asset {
  id: string;
  metadata: AssetMetadata;
  blob: Blob;
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

export interface File {
  kind: "File";
  id: string;
  path: string;
  asset: Asset | null;
}

export interface Folder {
  kind: "Folder";
  id: string;
  path: string;
}

export type Stem = Folder | File

export interface Location {
  path: string;
  stems: Array<Stem>;
}
