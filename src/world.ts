import { invoke } from "@tauri-apps/api/tauri"
import { File, Folder, Value, Location, Stem } from "./catalogue/value"
import createStore from "zustand"
import { Route } from "./aux/route"
// import createStore, { SetState } from "zustand"

export interface Store {
  world: World;
  locate: (route: Route) => void;
  setRoute: (route: Route) => void;
  getRoute: () => Route | null;
  focus: () => void;
  blur: () => void;
  isInFocus: () => boolean;
}

export const useStore = createStore<Store>((set, get) => ({
  // The world starts in the Void.
  world: { id: "void" },

  // Catalogue Actions
  locate: async (route: Route) => {
    try {
      const value: Value = await invoke("locate", { route })
      console.log("locate", value)

      set(state => ({ world: updateCatalogue(value, state.world) }))
    } catch (error) {
      console.error(error)
    }
  },
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

export function getFolders(catalogue: Catalogue): Array<Folder> {
  const { folders } = catalogue.current

  return folders
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
