import { invoke } from "@tauri-apps/api"
import { batch, createContext, createSignal, useContext } from "solid-js"
import { createStore, produce, reconcile, unwrap } from "solid-js/store"
import type { Route } from "../aux/route"
import type {
  FolderId,
  FolderDetails,
  Ground,
  Location,
  LocationFolders,
  LocationAssetPage,
  FolderMap,
  State,
  AssetStore,
  AssetCursor,
  AssetId,
  Thumbnail,
  ThumbnailStore,
  LocationId,
  AssetLocation,
  FolderLocation,
} from "../catalogue/types"

// TODO: start extract

async function fetchLocationFolders(id: FolderId): Promise<LocationFolders> {
  try {
    const state: LocationFolders = await invoke("locate_folders", { id })

    return state
  } catch (error) {
    console.log(error)
    const message = error as string
    throw (new FetchError(message, "locate"))
  }
}

async function fetchLocationAssetPage(id: FolderId, cursor: AssetCursor): Promise<LocationAssetPage> {
  try {
    const state: LocationAssetPage = await invoke("locate_asset_page", { id, cursor })

    return state
  } catch (error) {
    console.log(error)
    const message = error as string
    throw (new FetchError(message, "locate"))
  }
}

/** Fetches all pages from start to end */
async function fetchLocationAssets(parentId: FolderId): Promise<AssetStore> {
  const firstPage = await fetchLocationAssetPage(parentId, null)

  let cursor = firstPage.next_cursor
  let assets = firstPage.data

  while (cursor !== null) {
    let page = await fetchLocationAssetPage(parentId, cursor)
    assets = assets.concat(page.data)
    cursor = page.next_cursor
  }

  return ({ cursor, assets, parentId })
}

async function fetchLocation(id: FolderId): Promise<Location> {
  try {
    const state: Location = await invoke("locate", { id })

    return state
  } catch (error) {
    console.log(error)
    const message = error as string
    throw (new FetchError(message, "locate"))
  }
}

async function fetchThumbnail(id: AssetId): Promise<Thumbnail> {
  try {
    const state: Thumbnail = await invoke("fetch_thumbnail", { id })

    return state
  } catch (error) {
    console.log(error)
    const message = error as string
    throw (new FetchError(message, "fetch_thumbnail"))
  }
}

async function fetchGround(): Promise<Ground> {
  try {
    const state: Ground = await invoke("locate_ground")

    return state
  } catch (error) {
    console.log(error)
    const message = error as string
    throw (new FetchError(message, "fetchGround"))
  }
}

async function fetchFolderDetails(id: FolderId) {
  try {
    const state: FolderDetails = await invoke("fetch_folder_details", { id })

    return state
  } catch (error) {
    console.log(error)
    throw (new FetchError((error as Error).message, "fetchFolderDetails"))
  }
}





// TODO: end extract


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
  const [assetStore, setAssetStore] = createStore<AssetStore>({})
  const [state, setState] = createStore<State>({
    tree: {},
    isDetailsOpen: false,
    isBrowserFocused: false,
  })
  const [folderDetails, setFolderDetails] = createSignal<FolderDetails>()


  const restoreLocation = async (location: FolderLocation | AssetLocation) => {
    const ancestors = location.trail
    const parentId = ancestors[ancestors.length - 1]

    if (ground() === undefined) {
      setGround(await fetchGround())
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

    // Restoring assets is only required if the previous parentId is either
    // undefined or not the new location parent.
    const previousAssetParentId = assetStore.parentId
    if (location.kind === "Asset" && previousAssetParentId !== parentId) {
      const assetStore = await fetchLocationAssets(parentId)
      setAssetStore(reconcile(assetStore, { merge: false }))
    }
  }

  const resetLocation = () => {
    setFolderMap(reconcile({}))
    setAssetStore(reconcile({}))
    setState('tree', reconcile({}))
    setFolderDetails()
  }

  const navigate = (id: LocationId) => {
    if (location()?.id === id) { return }

    batch(async () => {
      const location: Location = await fetchLocation(id)
      setLocation(location)

      if (location.kind === "Ground") {
        setGround(await fetchGround())
        resetLocation()
      } else if (location.kind === "Folder") {
        const folders = await fetchLocationFolders(id)
        const assetStore = await fetchLocationAssets(id)
        const folderDetails = await fetchFolderDetails(id)

        restoreLocation(location)

        setFolderMap(id, folders)
        setAssetStore(reconcile(assetStore, { merge: false }))
        setFolderDetails(folderDetails)

      } else if (location.kind === "Asset") {
        restoreLocation(location)
      }
    })
  }

  const toggleTreeNode = (newId: FolderId) => {
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
      assetStore,

      // A route is the composition of scope e.g.`/catalogue` and path.
      // TODO: Must find a way to disambiguate that does not use UUIDs.
      route() {
        const currentLocation = location()

        if (currentLocation === undefined) {
          return "loading"
        } else if (currentLocation.kind === "Ground") {
          return "/catalogue/"
        } else {
          return `/catalogue${currentLocation.path}`
        }
      },
    },

    // write
    {
      navigate,
      fetchGround,
      fetchFolderDetails,
      toggleTreeNode,
      fetchThumbnail,

      toggleFolderDetails() {
        setState('isDetailsOpen', s => !s)
      },

      toggleBrowserFocus() {
        setState('isBrowserFocused', s => !s)
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
