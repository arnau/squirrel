const CatalogueScreen = lazy(() => import("./catalogue/CatalogueScreen"))
const PreferencesScreen = lazy(() => import("./preferences/PreferencesScreen"))
import { Navigate, Route, Routes, useNavigate } from "@solidjs/router"
import { appWindow } from "@tauri-apps/api/window"
import { lazy, onMount, onCleanup } from "solid-js"
import { type UnlistenFn, listen } from "@tauri-apps/api/event"
import { PreferencesData } from "./preferences/PreferencesContext"


export function App() {
  const navigate = useNavigate()
  let unlisten: UnlistenFn

  onMount(async () => {
    unlisten = await listen("navigate", async (event) => {
      const newRoute = `/${event.payload}`
      navigate(newRoute)
      await appWindow.setFocus()
    })
  })

  onCleanup(() => {
    unlisten()
  })

  return (
    <Routes>
      <Route path="/preferences/:id" component={PreferencesScreen} data={PreferencesData} />
      <Route path="/preferences/" element={<Navigate href="/preferences/general" />} />
      <Route path="/catalogue/:id?" component={CatalogueScreen} />
      <Route path="/" element={<Navigate href="/catalogue" />} />
    </Routes>
  )
}
