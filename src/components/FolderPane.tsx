import { Box, GridItem, Link, List, ListItem, Text, useStyleConfig } from "@chakra-ui/react"
import { Folder, Location } from "../catalogue/value"
import { MouseEvent, ReactNode } from "react"
import { lastSegment, Route } from "../aux/route"


export default function FolderPane({ location, roots, folders, onClick }: FolderPaneProps) {
  const currentRoot = location.stems[0]

  // It's /
  if (currentRoot == undefined) {
    return (
      <GridItem colSpan={1} rowSpan={5} bg="neutral" overflowY="auto" onClick={onClick}>
        {
          roots.map(root => <Root key={root.id} {...root} />)
        }
      </GridItem>
    )
  }

  return (
    <GridItem colSpan={1} rowSpan={5} bg="neutral" overflowY="auto" onClick={onClick}>
      {
        roots.map(root =>
          root.id == currentRoot.id
            ? <CurrentRoot key={root.id} folders={folders} {...root} />
            : <Root key={root.id} {...root} />)
      }
    </GridItem>
  )
}

interface FolderPaneProps {
  location: Location;
  roots: Array<Folder>;
  folders: Array<Folder>;
  onClick: (event: MouseEvent<HTMLElement>) => void;
}

function Root({ path }: Folder) {
  return (
    <Box>
      <NavLink href={path}>{path}</NavLink>
    </Box>
  )
}

interface CurrentRootProps {
  id: string;
  path: Route;
  folders: Array<Folder>;
}

function CurrentRoot({ path, folders }: CurrentRootProps) {
  return (
    <Box>
      <NavLink href={path}>{path}</NavLink>
      <FolderList folders={folders} />
    </Box>
  )
}

function FolderList({ folders }: { folders: Array<Folder> }) {
  return (
    folders.length > 0
      ? <List>{folders.map(folder => <FolderItem key={folder.id} {...folder} />)}</List>
      : <Text fontSize="small">(empty)</Text>
  )
}

function FolderItem({ path }: Folder) {
  const name = lastSegment(path)

  return (
    <ListItem>
      <NavLink href={path}>{name}</NavLink>
    </ListItem>
  )
}

function NavLink({ href, children }: { href: Route, children: ReactNode }) {
  const styles = useStyleConfig("NavLink")

  return (
    <Link
      sx={styles}
      onClick={(event: MouseEvent<HTMLElement>) => event.preventDefault()}
      tabIndex={0}
      href={href}
    >
      {children}
    </Link>
  )
}
