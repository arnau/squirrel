
// A route or path. E.g. `/2021/`, `/2021/foo/bar/`
export type Route = string

// A segment of a route. E.g. `2021`
export type Segment = string

// A route fragment. E.g. `2021/foo`
//
// Fragments are typically found when using `useParams().fragment`
export type Fragment = string

/**
  * Example:
  *
  * ```
  * segments("/2021/foo/") #=> ["2021", "foo"]
  * ```
  */
export function segments(route: Route): Array<Segment> {
  return route.split("/").filter(x => x != "")
}

export function lastSegment(route: Route): Segment | null {
  const xs = segments(route)
  const last = xs[xs.length - 1]

  return last ? last : null
}

/**
  * Example:
  *
  * ```
  * fromSegments(["2021", "foo"]) #=> "/2021/foo/"
  * ```
  */
export function fromSegments(segments: Array<Segment>): Route {
  if (segments.length === 0) { return "/" }

  return `/${segments.join('/')}/`
}

/**
  * Example:
  *
  * ```
  * fromSegments(["2021", "foo"]) #=> "/2021/foo/"
  * ```
  */
export function fromFragment(fragment: Fragment): Route {
  if (fragment.length === 0) { return "/" }

  return `/${fragment}/`
}

export function firstSegment(route: Route): Route | null {
  const xs = segments(route)
  const firstSegment = xs[0]

  if (firstSegment === undefined) { return null }

  return fromSegments([firstSegment])
}

export function isRootOf(a: Route, b: Route): boolean {
  return firstSegment(a) === b
}
