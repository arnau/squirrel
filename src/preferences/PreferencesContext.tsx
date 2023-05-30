import { invoke } from "@tauri-apps/api";
import { createContext, createResource, useContext } from "solid-js"

async function fetchSection(id: string) {
  const data = await invoke("fetch_pref_section", { id })

  return data
}

// Data function
export function PreferencesData({ params, location, navigate, data }: any) {
  const [section] = createResource(() => params.id, fetchSection);
  return section;
}


export const PreferencesContext = createContext()
export function PreferencesProvider(props: any) {

  const navigate = (id: string) => {
  }

  const value = [
    // read
    {
    },

    // write
    {
      navigate,
    }
  ]

  return (
    <PreferencesContext.Provider value={value} >
      {props.children}
    </PreferencesContext.Provider>
  )
}

export function usePreferences() { return useContext(PreferencesContext); }