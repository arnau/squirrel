import { Route } from "./aux/route"

export interface History {
  past: Array<Route>;
  present: Route;
  future: Array<Route>;
}


export function init(route: Route): History {
  return {
    past: [],
    present: route,
    future: [],
  }
}

/** Adds a new 'present' route pushing the previous route as 'past'.
  *
  * WARNING: It prunes any 'future' to avoid history branching.
  */
export function add(route: Route, history: History): History {
  const { past, present } = history

  past.push(present)

  return {
    past,
    present: route,
    future: [],
  }
}

export function back(history: History): History {
  const { past, present, future } = history

  // Nowhere to go back to
  if (past.length === 0) { return history }

  const newPresent = past.pop()
  future.push(present)

  return {
    past,
    present: newPresent!,
    future,
  }
}

export function forward(history: History): History {
  const { past, present, future } = history

  // Nowhere to go forward to
  if (future.length === 0) { return history }

  const newPresent = future.pop()
  past.push(present)

  return {
    past,
    present: newPresent!,
    future,
  }
}

export function isFirst(history: History): boolean {
  return history.past.length === 0
}

export function isLast(history: History): boolean {
  return history.future.length === 0
}
