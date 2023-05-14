import { invoke } from "@tauri-apps/api"
import { batch, createContext, createSignal, useContext } from "solid-js"
import { createStore, produce } from "solid-js/store"
import type { Route } from "../aux/route"
import type {
  EntryId,
  FolderDetails,
  Ground,
  Location,
  LocationFolders,
  LocationAssets,
  LocationAssetPage,
  FolderMap,
  State,
  AssetStore,
  AssetCursor,
} from "../catalogue/types"


export interface LocateError {
  oldRoute: Route,
  newRoute: Route,
  message: string,
}

class FetchError extends Error {
  action: string

  constructor(message: string, action: string, options: Object = {}) {
    super(message, options)

    this.action = action
  }

}

function sizeKB(data: any): number {
  const size = new TextEncoder().encode(JSON.stringify(data)).length

  return size / 1024
}



export const CatalogueContext = createContext()
export function CatalogueProvider(props: any) {
  const [location, setLocation] = createSignal<Location>()
  const [ground, setGround] = createSignal<Ground>()
  const [folderMap, setFolderMap] = createStore<FolderMap>({})
  const [state, setState] = createStore<State>({ tree: {}, isDetailsOpen: false })
  const [folderDetails, setFolderDetails] = createSignal<FolderDetails>()
  const [assets, setAssets] = createStore<AssetStore>({})

  const fetchGround = async () => {
    try {
      const state: Ground = await invoke("locate_ground")
      setGround(state)
      console.log(`ground KB`, sizeKB(state))

    } catch (error) {
      console.log(error)
      const message = error as string
      throw (new FetchError(message, "fetchGround"))
    }
  }

  const fetchLocation = async (id: EntryId): Promise<Location> => {
    try {
      const state: Location = await invoke("locate", { id })

      console.log(`location KB ${id}`, sizeKB(state))

      return state
    } catch (error) {
      console.log(error)
      const message = error as string
      throw (new FetchError(message, "locate"))
    }
  }

  const fetchLocationFolders = async (id: EntryId): Promise<LocationFolders> => {
    try {
      const state: LocationFolders = await invoke("locate_folders", { id })

      console.log(`location folders KB ${id}`, sizeKB(state))

      return state
    } catch (error) {
      console.log(error)
      const message = error as string
      throw (new FetchError(message, "locate"))
    }
  }

  const fetchLocationAssetPage = async (id: EntryId, cursor: AssetCursor): Promise<LocationAssetPage> => {
    try {
      const state: LocationAssetPage = await invoke("locate_page", { id, cursor })

      console.log(`location assets page KB ${id}`, sizeKB(state))

      return state
    } catch (error) {
      console.log(error)
      const message = error as string
      throw (new FetchError(message, "locate"))
    }
  }

  const fetchFolderDetails = async (id: EntryId) => {
    try {
      const state: FolderDetails = await invoke("fetch_folder_details", { id })
      console.log(`folder details KB ${id}`, sizeKB(state))
      setFolderDetails(state)
    } catch (error) {
      console.log(error)
      throw (new FetchError((error as Error).message, "fetchFolderDetails"))
    }
  }

  const locate = async (id: EntryId) => {
    const location = await fetchLocation(id)
    const folders = await fetchLocationFolders(id)
    // TODO: Add assets

    batch(() => {
      setFolderMap(location.id, folders)
      setLocation(location)
    })
  }

  // Fetch any missing ancestor.
  const rehydrate = async () => {
    const ancestors = location()!.trail

    if (ground() === undefined) {
      await fetchGround()
    }

    for (const ancestorId of ancestors) {
      if (folderMap[ancestorId] === undefined) {
        const folders = await fetchLocationFolders(ancestorId)

        setFolderMap(ancestorId, folders)
        setState('tree',
          produce(state => {
            state[ancestorId] = { isOpen: true }
          })
        )
      }
    }
  }

  const navigate = (newId: EntryId) => {
    if (location()?.id === newId) { return }

    if (newId === "") {
      batch(async () => {
        await fetchGround()
        setLocation({
          id: "",
          path: "/",
          trail: [],
        })
      })

    } else {
      batch(async () => {
        await locate(newId)
        await rehydrate()
        await fetchFolderDetails(newId)
          .catch((err) => console.error(err.message))
      })
    }
  }

  const toggleTreeNode = (newId: EntryId) => {
    batch(async () => {
      // Toggling might require an initial fetch if the data is not yet in place.
      if (folderMap[newId] === undefined) {
        const data = await fetchLocationFolders(newId)
        setFolderMap(newId, data)
      }

      setState("tree",
        produce(state => {
          state[newId] =
            (state[newId] === undefined)
              ? { isOpen: true }
              : { isOpen: !state[newId].isOpen }
        })
      )
    })
  }


  const value = [
    // read
    {
      location,
      ground,
      folderMap,
      state,
      folderDetails,

      // A route is the composition of scope e.g.`/catalogue` and path.
      // TODO: Must find a way to disambiguate that does not use UUIDs.
      route() {
        return location() === undefined
          ? `loading`
          : `/catalogue${location()?.path}`
      },
    },

    // write
    {
      navigate,
      fetchGround,
      fetchFolderDetails,
      toggleTreeNode,

      toggleFolderDetails() {
        setState('isDetailsOpen', s => !s)
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
