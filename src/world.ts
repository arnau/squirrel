import { createContext, useReducer, Reducer, Dispatch } from "react"
import { invoke } from "@tauri-apps/api/tauri"
import { File, Folder, Value, Location, Stem } from "./catalogue/value"


// The world's state identifier
export type WorldId = "pending" | "catalogue"

export type Action = {
  type: "locate" | "focus" | "blur" | "exception";
  payload?: Value;
}

export interface LocateAction {
  type: "locate";
  payload: Value;
}

export interface FocusAction {
  type: "focus" | "blur";
}

export interface BlameAction {
  type: "blame";
  message: string;
  // trace ?
}

// TODO: Replace reducer with this action shape.
export type Action2 = LocateAction | FocusAction | BlameAction


// A unique route or path.
export type Route = string

// The state of the world whilst waiting for data to be retrieved.
export interface Pending {
  id: "pending";
  route: Route;
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
  route: Route;
  current: Value;
  ui: Ui;
}

export interface Ui {
  isFocusMode: boolean;
}

export type World = Pending | Catalogue;


// A world starts with nothing. Pending the given route.
export function initWorld(route: Route): World {
  const world: World = {
    id: "pending",
    route,
  }

  return world
}

export function newCatalogue(route: Route, value: Value): World {
  return {
    id: "catalogue",
    route,
    current: value,
    ui: {
      isFocusMode: false,
    }
  }
}

export function updateCatalogue(newValue: Value, world: World): World {
  if (world.id == "pending") {
    return newCatalogue(world.route, newValue)
  } else {
    const newWorld = {
      ...world,
      current: newValue
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

export interface WorldBag {
  world: World;
  dispatch?: Dispatch<Action>;
}


export function useWorld(route: Route): WorldBag {
  const initialState = initWorld(route)
  const [world, dispatch] = useReducer<Reducer<World, Action>>(reducer, initialState)

  return ({ world, dispatch })
}

export function reducer(world: World, action: any) {
  switch (action.type) {
    case "locate":
      // return { ...world, current: action.payload }
      return updateCatalogue(action.payload, world)
    case "focus":
      if (world.id != "catalogue") {
        return world
      } else {
        return focus(world)
      }
    case "blur":
      if (world.id != "catalogue") {
        return world
      } else {
        return blur(world)
      }
    default:
      throw new Error("unknown action")
  }
}


export const Context = createContext<WorldBag>({
  world: initWorld("/"),
})



/** Extracts the route from the state.
  */
export function getRoute(world: World): Route {
  return world.route
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

export function locate(route: Route) {
  return invoke("locate", { route })
}

export function isInFocus(world: Catalogue): boolean {
  return world.ui.isFocusMode
}
