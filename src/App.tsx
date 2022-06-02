import React, { useState, ChangeEvent } from 'react'
import './App.css'
import { invoke } from '@tauri-apps/api/tauri'
import { createImage, revokeImage } from './aux/image'
import { Box, color, Flex, Text } from "@chakra-ui/react"
import { Input, InputGroup, InputLeftAddon, Button } from "@chakra-ui/react"
import { Grid, GridItem } from "@chakra-ui/react"

import LocatorBar from "./components/LocatorBar"


function App() {
  const [blob, setBlob] = useState<string | null>(null)
  const [location, setLocation] = useState('')
  const handleLocationChange =
    (event: ChangeEvent<HTMLInputElement>) => setLocation(event.target.value)

  return (
    <Grid
      gap={1}
      templateRows="50px repeat(4, 1fr)"
      templateColumns="repeat(6, 1fr)"
      height="100vh"
      bg="gray.800"
      minWidth={900}
    >
      <GridItem
        colSpan={6}
        rowSpan={1}
        bg="red.700"
        padding="8px 16px"
      >
        <LocatorBar
          location={location}
          onChange={handleLocationChange} />
      </GridItem>
      <GridItem colSpan={1} rowSpan={3} bg="neutral" />
      <GridItem
        colSpan={4}
        rowSpan={3}
        bg="neutral"
      >
      </GridItem>
      <GridItem colSpan={1} rowSpan={3} bg="neutral" />
      <GridItem colSpan={6} rowSpan={1} bg="neutral" />
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
