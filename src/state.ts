
export interface File {
  kind: "File";
  path: string;
}

export interface Folder {
  kind: "Folder";
  path: string;
}

export type Location = File | Folder


export interface State {
  location: Location;
}
