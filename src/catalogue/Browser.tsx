import styles from "./Browser.module.scss"

// import { ThumbPane } from "./ThumbPane"
// import FolderPane from "./FolderPane"
// import LocatorBar from "./LocatorBar"
// import { Catalogue, getAssets, getFolders, getRoots, getLocation, useStore, getCurrentStem } from "../world"
// import { Grid, GridItem } from "@chakra-ui/react"
// import { MaximiseButton } from "./ExpandToggle"
// import { MouseEvent } from "react"
// import { convertFileSrc } from "@tauri-apps/api/tauri"
//
//
/** Defines the main layout for browsing the catalogue.
 */
export function Browser() {
  // const world = useStore(state => state.world) as Catalogue
  // const locate = useStore(state => state.locate)
  // const add = useStore(state => state.add)
  // const roots = getRoots(world)
  // const folders = getFolders(world)
  // const assets = getAssets(world)
  // const location = getLocation(world)
  // const stem = getCurrentStem(location)
  //
  // const handleClick = (event: MouseEvent<HTMLElement>) => {
  //   const target = event.target as HTMLElement
  //
  //   if (target.nodeName == "A") {
  //     locate((target as HTMLAnchorElement).pathname)
  //   }
  // }

  return (
    <div class={styles.grid}>
      <LocatorPane />
      <TreePane />
      <ThumbPane />
      <AssetPane />
    </div>
  )
}

function LocatorPane() {
  // <LocatorBar />
  return (
    <div class={styles.locator_pane}></div>
  )
}
function TreePane() {
  return (
    <div class={styles.tree_pane}></div>
  )
}
function ThumbPane() {
  return (
    <div class={styles.thumb_pane}></div>
  )
}
function AssetPane() {
  return (
    <div class={styles.asset_pane}></div>
  )
}

// function AssetPane({ stem }: any) {
//   const focus = useStore(state => state.focus)
//
//   return (
//     <GridItem
//       colSpan={1}
//       rowSpan={5}
//       bg="neutral"
//       position="relative"
//     >
//       {
//         stem && stem.kind == "Asset"
//           ? <Asset
//             id={stem.id}
//             height={stem.metadata.height}
//             width={stem.metadata.width}
//             orientation={stem.metadata.orientation}
//             focus={focus} />
//           : null
//       }
//     </GridItem>
//   )
// }
//
// function Asset({ id, width, height, focus }: any) {
//   const url = convertFileSrc(`${id}.max`, "image")
//
//   return (
//     <div style={{
//       height: "100%",
//       width: "100%",
//       display: "flex",
//     }}>
//       <MaximiseButton setExpansion={focus} />
//       <img
//         src={url}
//         alt=""
//         height={height}
//         width={width}
//         style={{
//           display: "block",
//           objectFit: "contain",
//           // width: "100%",
//           // height: "100%",
//           maxHeight: "100%",
//           maxWidth: "100%",
//           margin: "auto",
//           alignItems: "center",
//         }}
//       />
//     </div>
//   )
// }
