import styles from "./Browser.module.scss"
import { A, useLocation, useParams } from "@solidjs/router"
import { TreePane } from "./TreePane"
import { createEffect } from "solid-js"
import { useCatalogue } from "./CatalogueContext"


/** Defines the main layout for browsing the catalogue.
 */
export function Browser() {
  // TODO: How to disambiguate between two roots with the same name (path)?
  //  - expose source? E.g. grounds could be a tree of sources + their respective roots.

  const params = useParams()
  const { pathname } = useLocation()
  const [, { navigate }]: any = useCatalogue()

  createEffect(() => {
    // TODO: Consider using data functions instead.
    console.log(pathname)
    const id = params.id ?? ""
    navigate(id)
  })

  return (
    <div class={styles.grid}>
      <LocatorPane />
      <TreePane />
      <AssetsPane />
      <ImagePane />
    </div>
  )
}

function LocatorPane() {
  const [{ route }]: any = useCatalogue()
  // <LocatorBar />
  return (
    <div class={styles.locator_pane}>
      <A href="/catalogue/">Catalogue</A>

      <span class={styles.location}>{route()}</span>
    </div>
  )
}


function AssetsPane() {
  return (
    <div class={styles.assets_pane}></div>
  )
}
function ImagePane() {
  return (
    <div class={styles.image_pane}></div>
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
