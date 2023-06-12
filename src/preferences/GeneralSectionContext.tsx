import { invoke } from "@tauri-apps/api";
import { createContext, useContext } from "solid-js"
import { createStore } from "solid-js/store";

interface GenearalSection {
  id: string,
  download_path: string,
}

export const GeneralSectionContext = createContext()
export function GeneralSectionProvider(props: any) {
  const [store, setStore] = createStore<GenearalSection>(props.value)

  const setDownloadPath = async (path: string) => {
    try {
      setStore("download_path", path)
      await invoke("store_preference", { key: "download_path", value: store.download_path })
    } catch (error) {
      console.error(error)
      throw error
    }
  }

  const value = [
    // read
    {
      store,
      downloadPath() { return store.download_path },
    },

    // write
    {
      setDownloadPath,
    }
  ]

  return (
    <GeneralSectionContext.Provider value={value} >
      {props.children}
    </GeneralSectionContext.Provider>
  )
}

export function useGeneralSection() { return useContext(GeneralSectionContext); }
