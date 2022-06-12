import { Browser } from "./Browser"
import { BrowserFocus } from "./BrowserFocus"
import { useStore } from "../world"


export function CatalogueScreen() {
  const isInFocus = useStore(state => state.isInFocus())

  return (
    isInFocus
      ? <BrowserFocus />
      : <Browser />
  )
}
