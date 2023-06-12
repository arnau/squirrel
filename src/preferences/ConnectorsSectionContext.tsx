import { invoke } from "@tauri-apps/api";
import { createContext, useContext } from "solid-js"
import { createStore } from "solid-js/store";

type ConnectorId = string

interface Connector {
  id: ConnectorId,
  key_name: string,
  bucket_name: string,
  secret_key: boolean,
  kind: string,
  creation_stamp: string,
}

interface ConnectorsSection {
  id: string,
  connectors: Connector,
}

export const ConnectorsSectionContext = createContext()
export function ConnectorsSectionProvider(props: any) {
  const [store, setStore] = createStore<ConnectorsSection>(props.value)

  // const setDownloadPath = async (path: string) => {
  //   try {
  //     setStore("download_path", path)
  //     await invoke("store_preference", { key: "download_path", value: store.download_path })
  //   } catch (error) {
  //     console.error(error)
  //     throw error
  //   }
  // }

  const value = [
    // read
    {
      store,
      connectorsList() { return store.connectors }
    },

    // write
    {
    }
  ]

  return (
    <ConnectorsSectionContext.Provider value={value} >
      {props.children}
    </ConnectorsSectionContext.Provider>
  )
}

export function useConnectorsSection() { return useContext(ConnectorsSectionContext); }
