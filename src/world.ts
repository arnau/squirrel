import { invoke } from "@tauri-apps/api/tauri"
import { Asset, Folder, Value, Location, Stem, AssetId } from "./catalogue/value"
import createStore from "zustand"
import { Route } from "./aux/route"
import type { History } from "./history"
import * as history from "./history"
// import createStore, { SetState } from "zustand"


export interface Cache {
  thumbnails: Map<string, string>;
}

function createCache() {
  return {
    thumbnails: new Map()
  }
}

export interface LocateResponse {
  status: "success" | "error";
  route: Route;
  message?: string;
}

export interface Store {
  world: World;
  cache: Cache;
  history: History;

  fetchThumbnail: (id: AssetId) => void;

  locate: (route: Route) => Promise<LocateResponse>;

  // history
  add: (route: Route) => void;
  back: () => Route;
  forward: () => Route;
  isFirst: () => boolean;
  isLast: () => boolean;
  getCurrentRoute: () => Route;
  getHistory: () => History;

  // locatorbar
  setRoute: (route: Route) => void;
  getRoute: () => Route | null;

  focus: () => void;
  blur: () => void;
  isInFocus: () => boolean;
}

export const useStore = createStore<Store>((set, get) => ({
  // The world starts in the Void.
  world: { id: "void" },
  cache: createCache(),
  history: history.init("/"),

  fetchThumbnail: async (id: AssetId) => {
    try {
      const data = get().cache.thumbnails.get(id)
      if (data) { return }

      const value: string = await invoke("thumbnail", { id })

      set(state => {
        const { thumbnails } = state.cache

        // Flush cache if it reaches the max cache capacity.
        const MAX_CACHE_SIZE = 2000
        if (thumbnails.size >= MAX_CACHE_SIZE) {
          thumbnails.clear()
        }

        thumbnails.set(id, value)

        return ({ cache: { ...state.cache, thumbnails } })
      })

    } catch (error) {
      console.error(error)
    }
  },

  // Catalogue Actions
  locate: async (route: Route): Promise<LocateResponse> => {
    try {
      const value: Value = await invoke("locate", { route })

      // TODO: handle error
      console.log(value)

      if (route !== get().history.present) {
        get().add(route)
      }

      set(state => ({ world: updateCatalogue(value, state.world) }))

      console.log(get().history)

      return ({ status: 'success', route })
    } catch (error) {
      const message = error as string
      console.log(error)

      return ({ status: "error", route, message })
    }
  },

  // history
  add: (route: Route) => {
    set(state => ({ history: history.add(route, state.history) }))
    console.log(get().history)
  },
  back: () => {
    set(state => ({ history: history.back(state.history) }))
    return get().history.present
  },
  forward: () => {
    set(state => ({ history: history.forward(state.history) }))
    return get().history.present
  },
  isFirst: () => {
    return history.isFirst(get().history)
  },
  isLast: () => {
    return history.isLast(get().history)
  },
  getCurrentRoute: () => {
    return get().history.present
  },
  getHistory: () => {
    return get().history
  },

  // location bar
  setRoute: (route) => {
    set(state => ({ world: updateRoute(route, state.world) }))
  },
  getRoute: () => {
    const world = get().world

    if (world.id == "catalogue") {
      return world.ui.newRoute
    }

    return null
  },
  focus: () => {
    set(state => ({ world: focus(state.world as Catalogue) }))
  },
  blur: () => {
    set(state => ({ world: blur(state.world as Catalogue) }))
  },
  isInFocus: () => isInFocus(get().world as Catalogue)
}))


// The world's state identifier
export type WorldId = "void" | "catalogue"

// The state of the world where nothing exist yet
export interface Void {
  id: "void";
}

// The state of the world after an exception.
//
// E.g. A request is sent to the backend and the response is unexpected.
export interface Exception {
  id: "exception";
  route: Route;
  message: string;
}

// The world as a catalogue.
export interface Catalogue {
  id: "catalogue";
  current: Value;
  ui: Ui;
}

export interface Ui {
  isFocusMode: boolean;
  // used by LocatorBar
  newRoute: Route;
}

export type World = Void | Catalogue;

export function initCatalogue(value: Value): World {
  return {
    id: "catalogue",
    current: value,
    ui: {
      newRoute: value.location.path,
      isFocusMode: false,
    }
  }
}

export function updateCatalogue(value: Value, world: World): World {
  if (world.id == "void") {
    return initCatalogue(value)
  } else {
    const newWorld = {
      ...world,
      current: value,
      ui: {
        ...world.ui,
        newRoute: value.location.path,
      }
    }

    return newWorld
  }
}


export function updateRoute(route: Route, world: World): World {
  if (world.id == "void") {
    return world
  } else {
    const newWorld = {
      ...world,
      ui: {
        ...world.ui,
        newRoute: route,
      }
    }

    return newWorld
  }
}

function focus(world: Catalogue): Catalogue {
  const newWorld = {
    ...world,
    ui: {
      ...world.ui,
      isFocusMode: true,
    }
  }

  return newWorld
}

function blur(world: Catalogue): Catalogue {
  const newWorld = {
    ...world,
    ui: {
      ...world.ui,
      isFocusMode: false,
    }
  }

  return newWorld
}


/** Extracts the route from the state.
  */
export function getRoute(world: World): Route | null {
  switch (world.id) {
    case "void":
      return null
    case "catalogue":
      return world.current.location.path
  }
}

export function getRoots(catalogue: Catalogue): Array<Folder> {
  const { roots } = catalogue.current

  return roots
}

export function getFolders(catalogue: Catalogue): Array<Folder> {
  const { folders } = catalogue.current

  return folders
}

export function getAssets(catalogue: Catalogue): Array<Asset> {
  const { assets } = catalogue.current

  return assets
}


export function getLocation(catalogue: Catalogue): Location {
  const { location } = catalogue.current

  return location
}

export function getCurrentStem(location: Location): Stem {
  const { stems } = location

  return stems[stems.length - 1]
}


/** Compares states by identity.
  */
export function compare(a: World, b: World): boolean {
  return getRoute(a) == getRoute(b)
}

export function isInFocus(world: Catalogue): boolean {
  return world.ui.isFocusMode
}
