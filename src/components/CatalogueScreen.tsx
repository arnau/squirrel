import { Browser } from "./Browser"
import { BrowserFocus } from "./BrowserFocus"
import { isInFocus, Context, Catalogue } from "../world"
import { useContext } from "react"


export function CatalogueScreen() {
  const { world } = useContext(Context)

  return (
    isInFocus((world as Catalogue))
      ? <BrowserFocus />
      : <Browser />
  )
}
