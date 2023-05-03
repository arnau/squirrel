// import { Catalogue, getCurrentStem, getLocation, useStore } from "../world"
// import { Grid, GridItem } from "@chakra-ui/react"
// import { MinimiseButton } from "./ExpandToggle"
// import { convertFileSrc } from "@tauri-apps/api/tauri"
//
//
// export function BrowserFocus() {
//   const world = useStore(state => state.world) as Catalogue
//   const location = getLocation(world)
//   const stem = getCurrentStem(location)
//
//   return (
//     <Grid
//       gap={1}
//       templateRows="repeat(6, 1fr)"
//       templateColumns="repeat(24, 1fr)"
//       height="100vh"
//       bg="gray.800"
//       minWidth={900}
//     >
//       <AssetPane stem={stem} />
//     </Grid>
//   )
// }
//
// function AssetPane({ stem }: any) {
//   const blur = useStore(state => state.blur)
//   return (
//     <GridItem colSpan={24} rowSpan={6} bg="neutral" position="relative">
//       <Asset
//         id={stem.id}
//         height={stem.metadata.height}
//         width={stem.metadata.width}
//         blur={blur}
//       />
//     </GridItem>
//   )
// }
//
// function Asset({ id, width, height, orientation, blur }: any) {
//   const url = convertFileSrc(`${id}.max`, "image")
//   const [w, h] = orientation == "AB" ? [width, height] : [height, width]
//
//   return (
//     <div style={{
//       overflow: "auto",
//       height: "100vh",
//     }}>
//       <MinimiseButton setExpansion={blur} />
//       <img
//         src={url}
//         alt=""
//         height={h}
//         width={w}
//         style={{
//           display: "block",
//         }}
//       />
//     </div>
//   )
// }
//
// function AssetDetailsPane() {
//   return (
//     <GridItem colSpan={22} rowSpan={1} bg="neutral">
//     </GridItem>
//   )
// }
