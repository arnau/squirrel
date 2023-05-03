import { useParams } from "@solidjs/router"
import styles from "./Catalogue.module.scss"
import { ErrorBoundary, createEffect } from "solid-js"
import { Browser } from "./Browser"

// TODO:
//
// - recreate state in navigator context
// - migrate App to Solidjs
// - migrate chackra to scss
// - migrate fake urls to Router
// - catch errors using solid error boundaries (e.g. locate fails, what should be displayed?)
export function CatalogueScreen() {
  const params = useParams()

  createEffect(() => {
    console.log('params', params.route)
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
