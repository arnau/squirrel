import { invoke } from "@tauri-apps/api"
import { batch, createContext, createSignal, useContext } from "solid-js"
import { createStore, produce } from "solid-js/store"
import { Fragment, fromFragment, type Route } from "../aux/route"
import { Folder, State, StateGround, StateTree, Tree, TreeNode } from "../catalogue/value"


export interface LocateError {
  oldRoute: Route,
  newRoute: Route,
  message: string,
}

type RootList = Array<Folder>

class FetchError extends Error {
  action: string

  constructor(message: string, action: string, options: Object = {}) {
    super(message, options)

    this.action = action
  }

}

export const CatalogueContext = createContext()
export function CatalogueProvider(props: any) {
  // current catalogue route.
  const [route, setRoute] = createSignal<Route>("/")
  const [roots, setRoots] = createSignal<RootList>()
  const [tree, setTree] = createStore<Tree>({ kind: "Empty" })
  const [state, setState] = createStore<State>({ tree: {}, isDetailsOpen: false })

  const fetchGround = async () => {
    try {
      const state: StateGround = await invoke("fetch_ground")

      setRoots(state.roots)

    } catch (error) {
      const message = error as string
      throw (new FetchError(message, "fetchGround"))
    }
  }

  const fetchRoot = async (newRoute: Route) => {
    try {
      const state: StateTree = await invoke("fetch_root", { path: newRoute })

      const size = new TextEncoder().encode(JSON.stringify(state)).length
      const kiloBytes = size / 1024;

      console.log('KB', kiloBytes)

      batch(() => {
        setTree(state.value)
        setRoute(newRoute)
      })

    } catch (error) {
      throw (new FetchError((error as Error).message, "fetchRoot"))
    }
  }

  const resetRoot = (newRoute: Route) => {
    batch(() => {
      setTree({ kind: "Empty", path: undefined, children: undefined })
      setState({ tree: {} })
      setRoute(newRoute)
    })
  }


  const value = [
    // read
    {
      route,
      roots,
      tree,
      state,
    },

    // write
    {
      fetchGround,
      fetchRoot,
      resetRoot,

      setRouteFromFragment(fragment: Fragment) {
        const newRoute = fromFragment(fragment)
        if (newRoute !== route()) {
          setRoute(newRoute)
        }
      },

      toggleFolderDetails() {
        setState('isDetailsOpen', s => !s)
      },

      toggleRoot(newRoute: Route) {
        (tree as TreeNode).path === newRoute
          ? resetRoot(newRoute)
          : fetchRoot(newRoute)
      },

      toggleTreeNode(newRoute: Route) {
        setState('tree',
          produce(treeState => {
            let state = treeState[newRoute]

            if (state === undefined) {
              treeState[newRoute] = { isOpen: true }
            } else {
              treeState[newRoute].isOpen = !state.isOpen
            }
          })
        )
      },
    }
  ]

  return (
    <CatalogueContext.Provider value={value} >
      {props.children}
    </CatalogueContext.Provider>
  )
}

export function useCatalogue() { return useContext(CatalogueContext); }
