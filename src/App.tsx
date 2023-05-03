import { Route, Routes } from "@solidjs/router";
import { CatalogueProvider } from "./catalogue/CatalogueContext";
import { NavigatorProvider } from "./navigator/NavigatorContext";
import { lazy } from "solid-js";
// import { CatalogueScreen } from "./catalogue/CatalogueScreen";
const CatalogueScreen = lazy(() => import("./catalogue/CatalogueScreen"));

export function App() {
  return (
    <NavigatorProvider>
      <CatalogueProvider>
        <Routes>
          <Route path="/catalogue/*route" component={CatalogueScreen} />
          <Route path="/settings" element={<div>settings, import screen, etc</div>} />
          <Route path="/" component={CatalogueScreen} />
        </Routes>
      </CatalogueProvider>
    </NavigatorProvider>
  )
}
