import React, { useState, ChangeEvent, Dispatch, SetStateAction } from 'react'
import './App.css'
import { invoke } from '@tauri-apps/api/tauri'
import { createImage, revokeImage } from './aux/image'
import { Grid, GridItem } from "@chakra-ui/react"

import { MaximiseButton, MinimiseButton } from "./components/ExpandToggle"
import LocatorBar, { LocatorBarProps } from "./components/LocatorBar"


function App() {
  const [expansion, setExpansion] = useState<boolean>(false)
  const [blob, setBlob] = useState<string | null>(null)
  const [location, setLocation] = useState('')
  const handleLocationChange =
    (event: ChangeEvent<HTMLInputElement>) => setLocation(event.target.value)

  return (
    expansion
      ? <CatalogueFocusLayout setExpansion={setExpansion} />
      : <CatalogueLayout setExpansion={setExpansion} locatorbar={{
        location,
        onChange: handleLocationChange,
      }} />
  )
}

type CatalogueLayoutProps = {
  locatorbar: LocatorBarProps;
  setExpansion: Dispatch<SetStateAction<boolean>>;
}

function CatalogueLayout({ setExpansion, locatorbar }: CatalogueLayoutProps) {
  return (
    <Grid
      gap={1}
      templateRows="50px repeat(5, 1fr)"
      templateColumns="repeat(6, 1fr)"
      height="100vh"
      bg="gray.800"
      minWidth={900}
    >
      <GridItem
        colSpan={6}
        rowSpan={1}
        bg="grey.700"
        padding="8px 16px"
      >
        <LocatorBar
          location={locatorbar.location}
          onChange={locatorbar.onChange} />
      </GridItem>
      <GridItem colSpan={1} rowSpan={5} bg="neutral" />
      <GridItem
        colSpan={2}
        rowSpan={5}
        bg="neutral"
      >
      </GridItem>
      <GridItem colSpan={3} rowSpan={4} bg="neutral" position="relative">
        <MaximiseButton setExpansion={setExpansion} />
      </GridItem>
      <GridItem colSpan={3} rowSpan={1} bg="neutral">
      </GridItem>
    </Grid>
  )
}

type CatalogueFocusLayoutProps = {
  setExpansion: Dispatch<SetStateAction<boolean>>;
}

function CatalogueFocusLayout({ setExpansion }: CatalogueFocusLayoutProps) {
  return (
    <Grid
      gap={1}
      templateRows="repeat(6, 1fr)"
      templateColumns="repeat(24, 1fr)"
      height="100vh"
      bg="gray.800"
      minWidth={900}
    >
      <GridItem colSpan={1} rowSpan={6} bg="neutral">
      </GridItem>
      <GridItem colSpan={22} rowSpan={5} bg="neutral" position="relative">
        <MinimiseButton setExpansion={setExpansion} />
      </GridItem>
      <GridItem colSpan={1} rowSpan={6} bg="neutral">
      </GridItem>
      <GridItem colSpan={22} rowSpan={1} bg="neutral">
      </GridItem>
    </Grid>
  )
}

function AppBack() {
  const [count, setCount] = useState(0)
  const [blob, setBlob] = useState<string | null>(null)

  return (
    <div className="App">
      <header className="App-header">
        <p>Squirrel</p>
        <p>
          <button type="button" onClick={() => setCount((count) => count + 1)}>
            count is: {count}
          </button>
          <button type="button" onClick={
            () => invoke('boost').then((n) => setCount((count) => count + (n as number)))
          }>
            boost!
          </button>
          <button type="button" onClick={
            () => invoke('state').then(raw => setBlob(createImage(raw as number[], 'image/png')))
          }>
            state
          </button>
          <button type="button" onClick={() => {
            revokeImage(blob);
            setBlob(null);
          }}>
            revoke
          </button>
        </p>
      </header>
      <main>
        <p>Bla bla</p>
        <p>
          Edit <code>App.tsx</code> and save to test HMR updates..
        </p>
        {
          (blob)
            ? <img src={blob} width="500" />
            : <span>noop</span>
        }
      </main>
    </div>
  )
}


export default App
