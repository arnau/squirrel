// import { Badge, Box, Grid, GridItem, Link, Text } from "@chakra-ui/react"
// import { MouseEvent, ReactNode, useRef } from "react"
// import { lastSegment, Route } from "../aux/route"
// import { useIntersection } from "../aux/useIntersection"
// import { Asset, AssetId, AssetMetadata, Location } from "../catalogue/value"
// import { useStore } from "../world"
//
//
// interface ThumbPaneProps {
//   location: Location;
//   assets: Array<Asset>;
// }
// export function ThumbPane({ location, assets }: ThumbPaneProps) {
//   // TODO: Instead of effect it should run after a folder click event.
//   // const elementRef = useRef<HTMLDivElement | null | undefined>()
//   // useEffect(() => {
//   //   if (elementRef !== null && elementRef !== undefined) {
//   //     elementRef.current!.scrollTo({
//   //       top: 0,
//   //       behavior: 'smooth',
//   //     })
//   //   }
//   // }, [])
//
//
//   return (
//     <GridItem
//       colSpan={1}
//       rowSpan={5}
//       background="neutral"
//       overflowY="auto"
//     // ref={el => elementRef.current = el}
//     >
//       {
//         assets.length == 0
//           ? <Text
//             marginTop={2}
//             textAlign="center"
//           >(empty folder)</Text>
//           : <Records list={assets} current_route={location.path} />
//       }
//     </GridItem>
//   )
// }
//
// interface RecordsProps {
//   list: Array<Asset>;
//   current_route: string;
// }
// function Records({ list, current_route }: RecordsProps) {
//   return (
//     <>
//       {
//         list.map(asset =>
//           <AssetItem
//             key={asset.id}
//             current_route={current_route}
//             {...asset} />
//         )
//       }
//     </>
//   )
// }
//
//
// type AssetItemProps = Asset & { current_route: Route }
// function AssetItem({ current_route, id, path, master_id, metadata }: AssetItemProps) {
//   const stem = lastSegment(path)
//
//   // Virtual assets are identified as a combination of the original asset path
//   // and the virtual asset copy.
//   // An original asset is identified just with the asset path.
//   const route = master_id
//     ? `${path}#${id}`
//     : path
//
//   const elementRef = useRef<HTMLAnchorElement | null>()
//   const isVisible = useIntersection(elementRef, '0px')
//
//   const selectedColour = route == current_route
//     ? "gray.700"
//     : "neutral"
//   const locate = useStore(state => state.locate)
//   const handleClick = (event: MouseEvent<HTMLElement>) => {
//     event.preventDefault()
//     event.stopPropagation()
//
//     const target = event.currentTarget as HTMLAnchorElement
//
//     locate(target.getAttribute("href")!)
//   }
//
//   return (
//     <Link
//       ref={el => elementRef.current = el}
//       id={id}
//       onClick={handleClick}
//       // tabIndex={0}
//       href={route}
//       sx={{
//         width: "300px",
//         height: "164px",
//         fontSize: "xs",
//         display: "block",
//         padding: "0",
//         background: selectedColour,
//         _hover: {
//           background: "gray.500",
//         }
//       }}
//     >
//       <Details
//         id={id}
//         stem={stem!}
//         metadata={metadata}
//         isVirtual={!!master_id}
//         isVisible={isVisible}
//       />
//     </Link>
//   )
// }
//
// interface DetailsProps {
//   id: AssetId;
//   stem: string;
//   metadata: AssetMetadata;
//   isVirtual: boolean;
//   isVisible: boolean;
// }
//
// function Details({ id, stem, metadata, isVirtual, isVisible }: DetailsProps) {
//   return (
//     <Grid
//       templateColumns="120px repeat(1, 1fr) repeat(1, 2fr)"
//       templateRows="repeat(4, 1fr)"
//     >
//       <Title>
//         {
//           isVirtual
//             ? <><Badge colorScheme="black">Virtual</Badge> {stem}</>
//             : stem
//         }
//       </Title>
//       <Thumbnail id={id} isVisible={isVisible} />
//
//       <Metapoint label="Format">{metadata.format}</Metapoint>
//       <Metapoint label="Stars">{metadata.rating}</Metapoint>
//       <Metapoint label="Flagged">{metadata.flag ? "Yes" : "No"}</Metapoint>
//       <Metapoint label="Colour">{metadata.label}</Metapoint>
//       <Metapoint label="Size" className="last">{`${metadata.width} x ${metadata.height}`}</Metapoint>
//     </Grid>
//   )
// }
//
// interface TitleProps {
//   children: ReactNode;
// }
//
// function Title({ children }: TitleProps) {
//   return (
//     <GridItem
//       sx={{
//         borderColor: "gray.800",
//         borderWidth: "0 0 1px 0",
//         borderStyle: "solid",
//         padding: "4px",
//       }}
//       colSpan={3}
//       rowSpan={1}
//     >
//       {children}
//     </GridItem>
//
//   )
// }
//
//
//
// interface ThumbnailProps {
//   id: AssetId;
//   isVisible: boolean;
// }
//
// function Thumbnail({ id, isVisible }: ThumbnailProps) {
//   const fetchThumbnail = useStore(state => state.fetchThumbnail)
//   const data = useStore(state => state.cache.thumbnails.get(id))
//   const url = dataUrl(data)
//
//   if (isVisible) {
//     fetchThumbnail(id)
//   }
//
//
//   const img =
//     <img
//       src={url}
//       // blocks the UI. image_protocol should be async
//       // src={`image://localhost/${id}.thumb`}
//       alt=""
//       style={{
//         display: "block",
//         objectFit: "contain",
//         margin: "auto",
//         alignItems: "center",
//         maxWidth: "100%",
//         maxHeight: "120px",
//       }}
//     />
//
//   return (
//     <GridItem
//       sx={{
//         borderColor: "gray.800",
//         borderWidth: "0 0 3px 0",
//         borderStyle: "solid",
//       }}
//       rowSpan={5}
//     >
//       <Box sx={{
//         display: "flex",
//         padding: "6px",
//         width: "100%",
//         height: "100%",
//       }}>
//         {img}
//       </Box>
//     </GridItem>
//   )
// }
//
// interface MetapointProps {
//   label: string;
//   className?: string;
//   children: ReactNode;
// }
//
// function Metapoint({ label, className, children }: MetapointProps) {
//   const width = className === "last"
//     ? '3px'
//     : '1px'
//   const cellStyles = {
//     borderColor: "gray.800",
//     borderWidth: `0 0 ${width} 1px`,
//     borderStyle: "solid",
//     padding: "4px",
//   }
//
//   return (
//     <>
//       <GridItem sx={cellStyles}>
//         {label}
//       </GridItem>
//       <GridItem sx={cellStyles}>
//         {children}
//       </GridItem>
//     </>
//   )
// }
//
// function dataUrl(data: string | undefined): string {
//   return data === undefined
//     ? ''
//     : `data:image/jpeg;base64,${data}`
// }
//
//
//
