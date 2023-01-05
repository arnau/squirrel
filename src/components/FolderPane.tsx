import { GridItem, Link, List, ListItem, Text, useStyleConfig } from "@chakra-ui/react"
import { Folder, Location } from "../catalogue/value"
import { MouseEvent, ReactNode } from "react"
import { lastSegment, Route } from "../aux/route"


interface FolderPaneProps {
  location: Location;
  roots: Array<Folder>;
  folders: Array<Folder>;
  onClick: (event: MouseEvent<HTMLElement>) => void;
}
export default function FolderPane({ folders, onClick }: FolderPaneProps) {
  return (
    <GridItem
      colSpan={1}
      rowSpan={5}
      bg="neutral"
      overflowY="auto"
      onClick={onClick}
    >
      <FolderList folders={folders} />
    </GridItem>
  )
}


function FolderList({ folders }: { folders: Array<Folder> }) {
  return (
    folders.length > 0
      ? <List>{folders.map(folder => <FolderItem key={folder.id} {...folder} />)}</List>
      : <Text textAlign="center" fontSize="small">(empty folder)</Text>
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
