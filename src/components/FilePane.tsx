import { Box, Grid, GridItem, Image, Link, Text } from "@chakra-ui/react"
import { MouseEvent } from "react"
import { Route } from "../aux/route"
import { Asset, Location } from "../catalogue/value"
import { useStore } from "../world"


export function FilePane({ location, assets, onClick }: FilePaneProps) {
  return (
    <GridItem
      colSpan={2}
      rowSpan={5}
      background="neutral"
      overflowY="auto"
      onClick={onClick}
    >
      {
        assets.length > 0
          ? assets.slice(0, 11).map(asset =>
            <AssetItem key={asset.id} current_route={location.path} {...asset} />)
          : <Text>(empty)</Text>
      }
    </GridItem>
  )
}

interface FilePaneProps {
  location: Location;
  assets: Array<Asset>;
  onClick: (event: MouseEvent<HTMLElement>) => void;
}


function AssetItem({ current_route, id, path, metadata }: Asset & { current_route: Route }) {
  const fetchThumbnail = useStore(state => state.fetchThumbnail)
  const data = useStore(state => state.cache.thumbnails.get(id))

  fetchThumbnail(id)

  const url = data === undefined
    ? ''
    : `data:image/jpeg;base64,${data}`

  const selectedColour = path == current_route
    ? "gray.700"
    : "neutral"
  const locate = useStore(state => state.locate)

  const cellStyles = {
    borderColor: "gray.800",
    borderWidth: "0 0 3px 3px",
    borderStyle: "solid",
  }

  return (
    <Link
      id={id}
      onClick={(event: MouseEvent<HTMLElement>) => {
        event.preventDefault()
        event.stopPropagation()

        const target = event.currentTarget as HTMLAnchorElement

        locate(target.pathname)

      }}
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
        templateColumns="80px repeat(1, 1fr)"
        templateRows="repeat(4, 1fr)"
      >
        <GridItem sx={{
          borderColor: "gray.800",
          borderWidth: "0 0 3px 0",
          borderStyle: "solid",
        }}
        rowSpan={5}
        >
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
        <GridItem sx={cellStyles}>
          {metadata.format}
        </GridItem>
        <GridItem sx={cellStyles}>
          {metadata.rating}
        </GridItem>
        <GridItem sx={cellStyles}>
          {JSON.stringify(metadata.flag)}
        </GridItem>
        <GridItem sx={cellStyles}>
          {metadata.label}
        </GridItem>
        <GridItem sx={cellStyles}>
          {`${metadata.width} x ${metadata.height}`}
        </GridItem>

      </Grid>
    </Link>
  )
}
