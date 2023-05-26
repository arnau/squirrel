import styles from "./Browser.module.scss"
import { A, useLocation, useParams } from "@solidjs/router"
import { TreePane } from "./TreePane"
import { createEffect } from "solid-js"
import { useCatalogue } from "./CatalogueContext"
import { ImagePane } from "./ImagePane"
import { AssetsPane } from "./AssetsPane"


/** Defines the main layout for browsing the catalogue.
 */
export function Browser() {
  // TODO: How to disambiguate between two roots with the same name (path)?
  //  - expose source? E.g. grounds could be a tree of sources + their respective roots.

  const params = useParams()
  const { pathname } = useLocation()
  const [, { navigate }]: any = useCatalogue()

  createEffect(() => {
    // TODO: Consider using data functions instead.
    console.log(pathname)
    const id = params.id ?? ""
    navigate(id)
  })

  return (
    <div class={styles.grid}>
      <LocatorPane />
      <TreePane />
      <AssetsPane />
      <ImagePane />
    </div>
  )
}

function LocatorPane() {
  const [{ route }]: any = useCatalogue()
  // <LocatorBar />
  return (
    <div class={styles.locator_pane}>
      <A href="/catalogue/">Catalogue</A>

      <span class={styles.location}>{route()}</span>
    </div>
  )
}
