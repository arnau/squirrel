import { invoke } from "@tauri-apps/api";
import { createContext, createSignal, useContext } from "solid-js"
import { createStore } from "solid-js/store";

type ConnectorId = string

interface NewConnector {
  id: ConnectorId,
  key_name: string,
  bucket_name: string,
  secret_key: string,
  kind: string,
}

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
  connectors: Array<Connector>,
}

export const ConnectorsSectionContext = createContext()
export function ConnectorsSectionProvider(props: any) {
  const [store, setStore] = createStore<ConnectorsSection>(props.value)

  // New Connector form
  const emptyForm = () => ({
    id: "",
    key_name: "",
    bucket_name: "",
    secret_key: "",
    kind: "backblaze",
  })
  const [form, setForm] = createStore<NewConnector>(emptyForm())
  const [formError, setFormError] = createSignal()
  const [actionErrors, setActionErrors] = createStore<object>({})

  const removeFromList = (connectorId: ConnectorId) =>
    setStore("connectors", store => store.filter(connector => connector.id !== connectorId))
  const addToList = (connector: Connector) =>
    setStore("connectors", store => [...store, connector])

  const storeConnector = async (newConnector: NewConnector) => {
    try {
      let connector: Connector = await invoke("store_connector", {connector: newConnector})
      addToList(connector)
      setForm(emptyForm())
    } catch (error) {
      setFormError(error)
    }
  }

  const removeConnector = async (connectorId: ConnectorId) => {
    try {
      await invoke("remove_connector", {connectorId})
      setActionErrors({[connectorId]: undefined})
      removeFromList(connectorId)
    } catch (error) {
      setActionErrors({[connectorId]: error})
    }
  }

  const value = [
    // read
    {
      store,
      connectorsList() { return store.connectors },
      form,
      formError,
      actionErrors,
    },

    // write
    {
      setForm,
      cancelForm() {
        setForm(emptyForm())
        setFormError(null)
      },
      submitForm() {
        storeConnector(form)
      },
      removeConnector,
    }
  ]

  return (
    <ConnectorsSectionContext.Provider value={value} >
      {props.children}
    </ConnectorsSectionContext.Provider>
  )
}

export function useConnectorsSection() { return useContext(ConnectorsSectionContext); }
