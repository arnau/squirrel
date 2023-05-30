import styles from "./Catalogue.module.scss"
import { ErrorBoundary } from "solid-js"
import { Browser } from "./Browser"
import { CatalogueProvider } from "./CatalogueContext"

export function CatalogueScreen() {
  return (
    <CatalogueProvider>
      <div class={styles.wrap}>
        <ErrorBoundary fallback={(err, reset) => <div onClick={reset}>{err}</div>}>
          <Browser />
        </ErrorBoundary>
      </div>
    </CatalogueProvider>
  )
}

export default CatalogueScreen
