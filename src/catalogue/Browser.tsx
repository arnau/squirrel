import { createEffect } from "solid-js"
import styles from "./Browser.module.scss"
import { useCatalogue } from "./CatalogueContext"
import { A, useParams } from "@solidjs/router"
import { TreePane } from "./TreePane"


/** Defines the main layout for browsing the catalogue.
 */
export function Browser() {
  const params = useParams()
  const [{ route }, { setRouteFromFragment }]: any = useCatalogue()

  createEffect(() => {
    setRouteFromFragment(params.fragment)
  })

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
  const [{ route }]: any = useCatalogue()
  // <LocatorBar />
  return (
    <div class={styles.locator_pane}>
      <A href="/catalogue/">Catalogue</A>

      <span class={styles.location}>Route: {route()}</span>
    </div>
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
