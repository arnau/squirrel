import { FilePane } from "./FilePane"
import FolderPane from "./FolderPane"
import LocatorBar from "./LocatorBar"
import { Catalogue, getFiles, getFolders, getRoots, getLocation, useStore } from "../world"
import { Grid, GridItem } from "@chakra-ui/react"
import { MaximiseButton } from "./ExpandToggle"
import { MouseEvent } from "react"


/** Defines the main layout for browsing the catalogue.
 */
export function Browser() {
  const world = useStore(state => state.world) as Catalogue
  const locate = useStore(state => state.locate)
  const roots = getRoots(world)
  const folders = getFolders(world)
  const files = getFiles(world)
  const location = getLocation(world)

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
      templateColumns="repeat(6, 1fr)"
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
      <FilePane
        location={location}
        files={files}
        onClick={handleClick}
      />

      <AssetPane />
      <AssetDetailsPane />
    </Grid>
  )
}

function LocatorPane() {
  return (
    <GridItem
      colSpan={6}
      rowSpan={1}
      bg="grey.700"
      padding="8px 16px"
    >
      <LocatorBar />
    </GridItem>
  )
}


function AssetPane() {
  const focus = useStore(state => state.focus)
  return (
    <GridItem colSpan={3} rowSpan={4} bg="neutral" position="relative">
      <MaximiseButton setExpansion={focus} />
    </GridItem>
  )
}

function AssetDetailsPane() {
  return (
    <GridItem colSpan={3} rowSpan={1} bg="neutral">
    </GridItem>
  )
}
