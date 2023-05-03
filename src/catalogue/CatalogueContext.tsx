import { invoke } from '@tauri-apps/api';
import { createContext, createSignal, onMount, useContext } from 'solid-js';
import { type Route } from '../aux/route';
import { Value } from '../catalogue/value';


export interface LocateError {
  route: Route,
  message: string,
}

type CatalogueItem = Value

export const CatalogueContext = createContext()
export function CatalogueProvider(props: any) {
  // current catalogue route.
  const [route, setRoute] = createSignal<Route>("/")
  // current catalogue item. TODO: Reshape and split.
  const [catalogueItem, setCatalogueItem] = createSignal<CatalogueItem>()

  onMount(async () => {
    await locate(route())
  })

  const locate = async (route: Route) => {
    try {
      const value: Value = await invoke("locate", { route })

      // TODO: handle error
      console.log(value)

      setCatalogueItem(value)
      setRoute(route)
    } catch (error) {
      const message = error as string

      console.log(error)

      throw ({ route, message } as LocateError)
    }
  }


  const value = [
    // read
    {
      route,
    },

    // write
    {
      locate,

    }
  ]

  return (
    <CatalogueContext.Provider value={value} >
      {props.children}
    </CatalogueContext.Provider>
  )
}

export function useNavigator() { return useContext(CatalogueContext); }
