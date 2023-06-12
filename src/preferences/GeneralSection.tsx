import { open } from "@tauri-apps/api/dialog"
import { downloadDir } from "@tauri-apps/api/path"
import styles from "./Preferences.module.scss"
import { useGeneralSection } from "./GeneralSectionContext"

export function GeneralSection() {
  return (
    <>
      <h1>General</h1>
      <DownloadSection />
    </>
  )
}

function DownloadSection() {
  const [{ downloadPath }, { setDownloadPath }]: any = useGeneralSection()

  const chooseHandler = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: await downloadDir(),
    })

    if (selected !== null) {
      setDownloadPath(selected as string)
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
          value={downloadPath()}
        />
        <button
          id="choose_folder"
          onClick={chooseHandler}
        >Choose…</button>
      </div>
    </section>
  )
}
