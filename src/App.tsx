import { Navigate, Route, Routes } from "@solidjs/router"
import { lazy } from "solid-js"
// import { CatalogueScreen } from "./catalogue/CatalogueScreen"
const CatalogueScreen = lazy(() => import("./catalogue/CatalogueScreen"))

export function App() {
  return (
    <Routes>
      <Route path="/catalogue/:id?" component={CatalogueScreen} />
      <Route path="/settings" element={<div>settings, import screen, etc</div>} />
      <Route path="/" element={<Navigate href="/catalogue" />} />
    </Routes>
  )
}
