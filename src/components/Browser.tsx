import { ThumbPane } from "./FilePane"
import FolderPane from "./FolderPane"
import LocatorBar from "./LocatorBar"
import { Catalogue, getAssets, getFolders, getRoots, getLocation, useStore, getCurrentStem } from "../world"
import { Grid, GridItem } from "@chakra-ui/react"
import { MaximiseButton } from "./ExpandToggle"
import { MouseEvent } from "react"
import { convertFileSrc } from "@tauri-apps/api/tauri"


/** Defines the main layout for browsing the catalogue.
 */
export function Browser() {
  const world = useStore(state => state.world) as Catalogue
  const locate = useStore(state => state.locate)
  const roots = getRoots(world)
  const folders = getFolders(world)
  const assets = getAssets(world)
  const location = getLocation(world)
  const stem = getCurrentStem(location)

  const handleClick = (event: MouseEvent<HTMLElement>) => {
    const target = event.target as HTMLElement

    if (target.nodeName == "A") {
      locate((target as HTMLAnchorElement).pathname)
    }
  }

  return (
    <Grid
      gap={1}
      templateRows="50px repeat(5, 1fr)"
      templateColumns="240px 300px minmax(600px, 1fr)"
      height="100vh"
      bg="gray.800"
      minWidth={900}
    >
      <LocatorPane />
      <FolderPane
        location={location}
        roots={roots}
        folders={folders}
        onClick={handleClick} />
      <ThumbPane
        location={location}
        assets={assets}
      />

      <AssetPane stem={stem} />
    </Grid>
  )
}

function LocatorPane() {
  return (
    <GridItem
      colSpan={3}
      rowSpan={1}
      bg="grey.700"
      padding="8px 16px"
    >
      <LocatorBar />
    </GridItem>
  )
}


function AssetPane({ stem }: any) {
  const focus = useStore(state => state.focus)

  return (
    <GridItem
      colSpan={1}
      rowSpan={5}
      bg="neutral"
      position="relative"
    >
      {
        stem && stem.kind == "Asset"
          ? <Asset
            id={stem.id}
            height={stem.metadata.height}
            width={stem.metadata.width}
            orientation={stem.metadata.orientation}
            focus={focus} />
          : null
      }
    </GridItem>
  )
}

function Asset({ id, width, height, focus }: any) {
  const url = convertFileSrc(`${id}.max`, "image")

  return (
    <div style={{
      height: "100%",
      width: "100%",
      display: "flex",
    }}>
      <MaximiseButton setExpansion={focus} />
      <img
        src={url}
        alt=""
        height={height}
        width={width}
        style={{
          display: "block",
          objectFit: "contain",
          // width: "100%",
          // height: "100%",
          maxHeight: "100%",
          maxWidth: "100%",
          margin: "auto",
          alignItems: "center",
        }}
      />
    </div>
  )
}
