import { Context } from "../world"
import { Grid, GridItem } from "@chakra-ui/react"
import { MinimiseButton } from "./ExpandToggle"
import { useContext } from "react"


export function BrowserFocus() {
  return (
    <Grid
      gap={1}
      templateRows="repeat(6, 1fr)"
      templateColumns="repeat(24, 1fr)"
      height="100vh"
      bg="gray.800"
      minWidth={900}
    >
      <PreviousPane />
      <AssetPane />
      <NextPane />
      <AssetDetailsPane />
    </Grid>
  )
}

function PreviousPane() {
  return (
    <GridItem colSpan={1} rowSpan={6} bg="neutral">
    </GridItem>
  )
}

function NextPane() {
  return (
    <GridItem colSpan={1} rowSpan={6} bg="neutral">
    </GridItem>
  )
}

function AssetPane() {
  const { dispatch } = useContext(Context)
  return (
    <GridItem colSpan={22} rowSpan={5} bg="neutral" position="relative">
      <MinimiseButton setExpansion={() => dispatch?.({ type: "blur" })} />
    </GridItem>

  )
}

function AssetDetailsPane() {
  return (
    <GridItem colSpan={22} rowSpan={1} bg="neutral">
    </GridItem>
  )
}
