import LocatorBar from "./LocatorBar"
import { Catalogue, Context, getCurrentStem, getFolders, getLocation } from "../world"
import { Grid, GridItem, useTheme } from "@chakra-ui/react"
import { MaximiseButton } from "./ExpandToggle"
import { useContext } from "react"
import { ThemeContext } from "@emotion/react"
import { Folder } from "../catalogue/value"

/** Defines the main layout for browsing the catalogue.
 */
export function Browser() {
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
      <FolderPane />
      <ThumbnailPane />

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

function FolderPane() {
  const { world } = useContext(Context)
  const folders = getFolders(world as Catalogue)
  const location = getLocation(world as Catalogue)
  const currentStem = getCurrentStem(location)

  console.log(currentStem)

  return (
    <GridItem colSpan={1} rowSpan={5} bg="neutral" overflowY="auto">
      <FolderList folders={folders} />
    </GridItem>
  )
}

interface FolderListProps {
  folders: Array<Folder>;
}

function FolderList({ folders }: FolderListProps) {
  const fs = folders.map(folder => {
    return <li key={folder.id} style={{ color: "whitesmoke" }}>{folder.path}</li>
  })

  return (
    folders.length == 0
      ? null
      : <ul>
        {fs}
      </ul>
  )
}

function AssetPane() {
  const { dispatch } = useContext(Context)
  return (
    <GridItem colSpan={3} rowSpan={4} bg="neutral" position="relative">
      <MaximiseButton setExpansion={() => dispatch?.({ type: "focus" })} />
    </GridItem>
  )
}

function ThumbnailPane() {
  return (
    <GridItem
      colSpan={2}
      rowSpan={5}
      bg="neutral"
    >
    </GridItem>
  )
}

function AssetDetailsPane() {
  return (
    <GridItem colSpan={3} rowSpan={1} bg="neutral">
    </GridItem>
  )
}
