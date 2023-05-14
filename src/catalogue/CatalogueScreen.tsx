import styles from "./Catalogue.module.scss"
import { ErrorBoundary, onMount } from "solid-js"
import { Browser } from "./Browser"

export function CatalogueScreen() {
  return (
    <div class={styles.wrap}>
      <ErrorBoundary fallback={(err, reset) => <div onClick={reset}>{err}</div>}>
        <Browser />
      </ErrorBoundary>
    </div>
  )
}

export default CatalogueScreen
