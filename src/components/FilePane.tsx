import { Box, Grid, GridItem, Image, Link, Text } from "@chakra-ui/react"
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { MouseEvent } from "react";
import { Route } from "../aux/route";
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
          ? files.map(file =>
            <FileItem key={file.id} current_route={location.path} {...file} />)
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


function FileItem({ current_route, id, path }: File & { current_route: Route }) {
  const url = convertFileSrc(`${id}.thumb`, "image")

  const selectedColour = path == current_route
    ? "gray.700"
    : "neutral"

  return (
    <Link
      id={id}
      onClick={(event: MouseEvent<HTMLElement>) => event.preventDefault()}
      tabIndex={0}
      href={path}
      sx={{
        fontSize: "sm",
        display: "block",
        padding: "0",
        background: selectedColour,
        _hover: {
          background: "gray.500",
        }
      }}
    >
      <Grid
        templateColumns="80px auto"
      >
        <GridItem border="1px solid black">
          <Box width="80px" height="100px" style={{
            display: "flex",
          }}>
            <img
              src={url}
              alt=""
              style={{
                display: "block",
                objectFit: "contain",
                margin: "auto",
                alignItems: "center",
                maxWidth: "100%",
                maxHeight: "100%",
              }}
            />
          </Box>
        </GridItem>
        <GridItem border="1px solid black">
          {path}
        </GridItem>
      </Grid>
    </Link>
  )
}
