import styles from "./Catalogue.module.scss"
import { ErrorBoundary, onMount } from "solid-js"
import { Browser } from "./Browser"
import { useCatalogue } from "./CatalogueContext"

// TODO:
//
// - recreate state in navigator context
// - migrate App to Solidjs
// - migrate chackra to scss
// - catch errors using solid error boundaries (e.g. locate fails, what should be displayed?)


// TODO locate("/") should fetch location as well as any data to display
// on this path.
export function CatalogueScreen() {
  const [, {fetchGround}]: any = useCatalogue()

  onMount(async () => {
    await fetchGround()
  })

  return (
    <div class={styles.wrap}>
      <ErrorBoundary fallback={(err, reset) => <div onClick={reset}>{err}</div>}>
        <Browser />
      </ErrorBoundary>
    </div>
  )
}

export default CatalogueScreen
