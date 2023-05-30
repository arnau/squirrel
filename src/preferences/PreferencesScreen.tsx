import { ErrorBoundary, Match, Switch, createEffect, createSignal } from "solid-js"
import styles from "./Preferences.module.scss"
import { A, useParams, useRouteData } from "@solidjs/router"
import { PreferencesProvider } from "./PreferencesContext"
import { downloadDir } from "@tauri-apps/api/path"
import { open } from "@tauri-apps/api/dialog"

export function PreferencesScreen() {
  const data: any = useRouteData()

  return (
    <PreferencesProvider>
      <div class={styles.layout}>
        <ErrorBoundary fallback={(err, reset) => <div onClick={reset}>{err}</div>}>
          <MenuPane />
          <DetailPane data={data()} />
        </ErrorBoundary>
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
      <h1>{props.data.title}</h1>
      <Switch>
        <Match when={props.data.id === "general"}>
          <GeneralSection {...props.data} />
        </Match>
        <Match when={props.data.id === "connectors"}>
          <div>TODO: connectors</div>
        </Match>
        <Match when={props.data.id === "sources"}>
          <div>TODO: sources</div>
        </Match>
      </Switch>
    </main>
  )
}

function GeneralSection(props: any) {
  return (
    <DownloadSection folder={props.download_folder} />
  )
}

function DownloadSection(props: any) {
  // TODO: Move to PreferencesContext
  const [downloadDirPath, setDownloadDirPath] = createSignal(props.folder)

  // createEffect(async () => {
  //   // TODO: Fetch info in the backend
  //   setDownloadDirPath(await downloadDir())
  // })

  const chooseHandler = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: await downloadDir(),
    })

    if (selected !== null) {
      setDownloadDirPath(selected as string)
    }
  }

  return (
    <section class={styles.downloads_section}>
      <h2>Downloads</h2>

      <div class={styles.group}>
        <label id="save_to" aria-control="download_folder">Save files to</label>
        <input
          id="download_folder"
          type="text"
          readonly={true}
          aria-labelledby="save_to"
          value={downloadDirPath()}
        // TODO: background-image folder icon (?)
        />
        <button
          id="choose_folder"
          onClick={chooseHandler}
        >Choose…</button>
      </div>
    </section>
  )
}
