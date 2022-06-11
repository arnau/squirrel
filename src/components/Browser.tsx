import LocatorBar from "./LocatorBar"
import { Catalogue, getCurrentStem, getFolders, getLocation, useStore } from "../world"
import { Grid, GridItem, Link, List, ListItem } from "@chakra-ui/react"
import { MaximiseButton } from "./ExpandToggle"
import { Folder } from "../catalogue/value"
import { lastSegment, Route } from "../aux/route"

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
  const world = useStore(state => state.world)
  const locate = useStore(state => state.locate)
  const folders = getFolders(world as Catalogue)
  const location = getLocation(world as Catalogue)
  const currentStem = getCurrentStem(location)

  console.log(currentStem)

  return (
    <GridItem colSpan={1} rowSpan={5} bg="neutral" overflowY="auto">
      <FolderList folders={folders} locate={locate} />
    </GridItem>
  )
}

interface FolderListProps {
  folders: Array<Folder>;
  locate: (route: Route) => void;
}

function FolderList({ folders, locate }: FolderListProps) {
  const list = folders.map(folder => {
    return <FolderItem key={folder.id} locate={locate} {...folder} />
  })

  return (
    folders.length > 0
      ? <List>{list}</List>
      : null
  )
}

interface FolderItemProps {
  id: string;
  path: Route;
  locate: (route: Route) => void;
}

function FolderItem({ path, locate }: FolderItemProps) {
  const name = lastSegment(path)

  return (
    <ListItem style={{ color: "whitesmoke" }}>
      <Link onClick={event => {
        event.preventDefault()
        locate(path)
      }}>{name}</Link>
    </ListItem>
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
