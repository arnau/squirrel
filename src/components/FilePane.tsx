import { Badge, Box, Grid, GridItem, Link, Text } from "@chakra-ui/react"
import { MouseEvent, ReactNode, useRef } from "react"
import { Route } from "../aux/route"
import { useIntersection } from "../aux/useIntersection"
import { Asset, Location } from "../catalogue/value"
import { useStore } from "../world"


export function ThumbPane({ location, assets }: ThumbPaneProps) {
  return (
    <GridItem
      colSpan={1}
      rowSpan={5}
      background="neutral"
      overflowY="auto"
    >
      {
        assets.length > 0
          ? assets.map(asset =>
            <AssetItem
              key={asset.id}
              current_route={location.path}
              {...asset} />
          )
          : <Text>(empty)</Text>
      }
    </GridItem>
  )
}

interface ThumbPaneProps {
  location: Location;
  assets: Array<Asset>;
}




const dummy = `data:image/jpeg;base64,/9j/4AAQSkZJRgABAQEASABIAAD/2wBDAAQDAwQDAwQEAwQFBAQFBgoHBgYGBg0JCggKDw0QEA8NDw4RExgUERIXEg4PFRwVFxkZGxsbEBQdHx0aHxgaGxr/2wBDAQQFBQYFBgwHBwwaEQ8RGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhr/wAARCAB4AHgDASIAAhEBAxEB/8QAGAABAQEBAQAAAAAAAAAAAAAAAAUGAQP/xAAqEAEAAAQFBAMAAQUAAAAAAAAAAgMzcgQ2dbPjAQW0tRIxQvARFjSEkf/EABQBAQAAAAAAAAAAAAAAAAAAAAD/xAAUEQEAAAAAAAAAAAAAAAAAAAAA/9oADAMBAAIRAxEAPwDP4nETOwT5s/FTI5/Z5sUUcyOOL5dcFFF9xRdf1J2rKVoQcq6B6/g2rKQXgAAAAAAAAAAQc1aB7Dg3bKoemGxEzv8APlT8LMjkdnlRQxy44Ivj1xsUP1FD1/MndsqloAABByroHr+DaspXhByroHr+DaspBeAAAAAABBzVoHsODdsqgzVoHsODdsq3gAAAAAABByroHr+DaspXhByroHr+DaspBeAABBzVoHsODdsqgzVoHsODdsq3gAAAAAAAAAABByroHr+DaspXhgf+f2B/T+f6X8/xwaDNWgew4N2yreAAAAAAAAAAAAEDNWgefwbtlUO5q0D2HBu2VbwAg5V0D1/BtWUrwgZV0DwODaspBfAAAAAAAABAzVoHn8G7ZVBmrQPP4N2yrfAAAAAEDKugeBwbVlK+IGVdA8Dg2rKQXwAAAAQM1aB5/Bu2VQZq0Dz+Ddsq3wAAAAAAAABAyroHgcG1ZSviBlXQPA4NqykF8AETEyJnf582RipcyR2eVFFBMgjh+PXGxQ/cMXT8yd2yrbAAAAAAAAAAAAAETDSJnYJ8qRhZcyf2ebFDBLggh+XXBRRfUMPT9SdqykAH/9k=`

function dataUrl(data: string | undefined): string {
  return data === undefined
    ? ''
    : `data:image/jpeg;base64,${data}`
}

function takeStem(path: string): string {
  const stems = path.split("/")

  return stems[stems.length - 1]
}


type AssetItemProps = Asset & { current_route: Route }
function AssetItem({ current_route, id, path, master_id, metadata }: AssetItemProps) {
  const fetchThumbnail = useStore(state => state.fetchThumbnail)
  const data = useStore(state => state.cache.thumbnails.get(id))
  const stem = takeStem(path)

  // Virtual assets are identified as a combination of the original asset path
  // and the virtual asset copy.
  // An original asset is identified just with the asset path.
  const route = master_id
    ? `${path}#${id}`
    : path

  const elementRef = useRef<HTMLAnchorElement | null>()
  const isVisible = useIntersection(elementRef, '0px')

  if (isVisible) {
    fetchThumbnail(id)
  }

  const url = dataUrl(data)

  const selectedColour = route == current_route
    ? "gray.700"
    : "neutral"
  const locate = useStore(state => state.locate)

  return (
    <Link
      ref={el => elementRef.current = el}
      id={id}
      onClick={(event: MouseEvent<HTMLElement>) => {
        event.preventDefault()
        event.stopPropagation()

        const target = event.currentTarget as HTMLAnchorElement

        locate(target.getAttribute("href")!)
      }}
      tabIndex={0}
      href={route}
      sx={{
        fontSize: "xs",
        display: "block",
        padding: "0",
        background: selectedColour,
        _hover: {
          background: "gray.500",
        }
      }}
    >
      <Grid
        templateColumns="120px repeat(1, 1fr) repeat(1, 2fr)"
        templateRows="repeat(4, 1fr)"
      >
        <GridItem
          sx={{
            borderColor: "gray.800",
            borderWidth: "0 0 1px 0",
            borderStyle: "solid",
            padding: "4px",
          }}
          colSpan={3}
          rowSpan={1}
        >
          {
            master_id
              ? <><Badge colorScheme="black">Virtual</Badge> {stem}</>
              : stem
          }
        </GridItem>
        <GridItem
          sx={{
            borderColor: "gray.800",
            borderWidth: "0 0 3px 0",
            borderStyle: "solid",
          }}
          rowSpan={5}
        >
          <Box sx={{
            display: "flex",
            padding: "6px",
            width: "100%",
            height: "100%",
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
                maxHeight: "120px",
              }}
            />
          </Box>
        </GridItem>

        <Metapoint label="Format">{metadata.format}</Metapoint>
        <Metapoint label="Stars">{metadata.rating}</Metapoint>
        <Metapoint label="Flagged">{metadata.flag ? "Yes" : "No"}</Metapoint>
        <Metapoint label="Colour">{metadata.label}</Metapoint>
        <Metapoint label="Size" className="last">{`${metadata.width} x ${metadata.height}`}</Metapoint>
      </Grid>
    </Link>
  )
}

interface MetapointProps {
  label: string;
  className?: string;
  children: ReactNode;
}

function Metapoint({ label, className, children }: MetapointProps) {
  const width = className === "last"
    ? '3px'
    : '1px'
  const cellStyles = {
    borderColor: "gray.800",
    borderWidth: `0 0 ${width} 1px`,
    borderStyle: "solid",
    padding: "4px",
  }

  return (
    <>
      <GridItem sx={cellStyles}>
        {label}
      </GridItem>
      <GridItem sx={cellStyles}>
        {children}
      </GridItem>
    </>
  )
}
