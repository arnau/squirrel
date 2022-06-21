import { Box, Grid, GridItem, Image, Link, Text } from "@chakra-ui/react"
import { MouseEvent } from "react";
import { File, Location } from "../catalogue/value"


export function FilePane({ location, files, onClick }: FilePaneProps) {
  const bg = files.length > 0 ? "gray.800" : "neutral"
  return (
    <GridItem
      colSpan={2}
      rowSpan={5}
      background={bg}
      overflowY="auto"
      onClick={onClick}
    >
      {
        files.length > 0
          ? files.map(file => <FileItem key={file.id} {...file} />)
          : <Text>(empty)</Text>
      }
    </GridItem>
  )
}

interface FilePaneProps {
  location: Location;
  files: Array<File>;
  onClick: (event: MouseEvent<HTMLElement>) => void;
}


function FileItem({ id, path, asset }: File) {
  //TODO: ensure asset metadata exists.
  // console.log(asset)

  return (
    <Grid
      templateColumns="80px auto"
      marginBottom="2px"
    >
      <GridItem bg="tan">
        <Box width="80px" height="100px" bg="tomato">
          loading
        </Box>
      </GridItem>
      <GridItem
        background="neutral"
      >
        <Link
          id={id}
          onClick={(event: MouseEvent<HTMLElement>) => event.preventDefault()}
          tabIndex={0}
          href={path}
        >
          {path}
        </Link>
      </GridItem>
    </Grid>
  )
}
