import { invoke } from '@tauri-apps/api';
import { createContext, createSignal, onMount, useContext } from 'solid-js';
import { type Route } from '../aux/route';


// TODO: Rething the responsibility of this context. Might be better to compose this as a
// reactive store in each screen context? or just have a fat context.
export const NavigatorContext = createContext()
export function NavigatorProvider(props: any) {
  // current route.
  const [route, setRoute] = createSignal<Route>("/")

  const value = [
    // read
    {
      route,
    },

    // write
    {

    }
  ]

  return (
    <NavigatorContext.Provider value={value} >
      {props.children}
    </NavigatorContext.Provider>
  )
}

export function useNavigator() { return useContext(NavigatorContext); }
