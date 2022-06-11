
// A unique route or path.
export type Route = string

type Segment = string;

export function segments(route: Route): Array<Segment> {
  return route.split("/").filter(x => x != "")
}

export function lastSegment(route: Route): Segment | null {
  const xs = segments(route)
  const last = xs[xs.length - 1]

  return last ? last : null
}
