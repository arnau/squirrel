import { ErrorBoundary, Match, Switch, createEffect } from "solid-js"
import styles from "./Preferences.module.scss"
import { A, useLocation, useRouteData } from "@solidjs/router"
import { PreferencesProvider } from "./PreferencesContext"
import { GeneralSection } from "./GeneralSection"
import { GeneralSectionProvider } from "./GeneralSectionContext"
import { appWindow } from "@tauri-apps/api/window"
import { ConnectorsSection } from "./ConnectorsSection"
import { SourcesSection } from "./SourcesSection"
import { ConnectorsSectionProvider } from "./ConnectorsSectionContext"


//TODO: Solid.js 1.7.x broke ErrorBoundary.
//<ErrorBoundary fallback={(err, reset) => <div onClick={reset}>{err}</div>}>
export function PreferencesScreen() {
  const data: any = useRouteData()
  const location = useLocation()

  createEffect(() => {
    appWindow.setTitle(location.pathname)
  })

  return (
    <PreferencesProvider>
      <div class={styles.layout}>
        <MenuPane />
        <DetailPane data={data()} />
      </div>
    </PreferencesProvider>
  )
}

export default PreferencesScreen

function MenuPane() {
  return (
    <div class={styles.menu_pane}>
      <ul>
        <li><A href="/preferences/general">General</A></li>
        <li><A href="/preferences/connectors">Connectors</A></li>
        <li><A href="/preferences/sources">Sources</A></li>
      </ul>
    </div>
  )
}


function DetailPane(props: any) {
  return (
    <main class={styles.detail_pane}>
      <Switch>
        <Match when={props.data?.id === "general"}>
          <GeneralSectionProvider value={props.data}>
            <GeneralSection />
          </GeneralSectionProvider>
        </Match>
        <Match when={props.data?.id === "connectors"}>
          <ConnectorsSectionProvider value={props.data}>
            <ConnectorsSection />
          </ConnectorsSectionProvider>
        </Match>
        <Match when={props.data?.id === "sources"}>
          <SourcesSection />
        </Match>
      </Switch>
    </main>
  )
}
