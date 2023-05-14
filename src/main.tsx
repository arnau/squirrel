// @refresh reload
import { render } from "solid-js/web"
import { Router } from "@solidjs/router"
import { App } from "./App"
import { NavigatorProvider } from "./navigator/NavigatorContext"
import { CatalogueProvider } from "./catalogue/CatalogueContext"

render(
  () => (
    <Router>
      <NavigatorProvider>
        <CatalogueProvider>
          <App />
        </CatalogueProvider>
      </NavigatorProvider>
    </Router>
  ),
  document.getElementById("root") as HTMLElement
)
