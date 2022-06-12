import { ReactElement, ChangeEvent, KeyboardEvent, MouseEvent, useState } from "react"
import { Flex } from "@chakra-ui/react"
import { Input, Button } from "@chakra-ui/react"
import { useStore } from "../world"
import { Route } from "../aux/route"


export default function LocatorBar(): ReactElement {
  const locate = useStore(state => state.locate)
  const setRoute = useStore(state => state.setRoute)
  const route = useStore(state => state.getRoute()) as Route

  const handleSubmit =
    (_event: MouseEvent<HTMLButtonElement>) => {
      locate(route)
  }
  const handleKeyChange =
    (event: ChangeEvent<HTMLInputElement>) => setRoute(event.target.value)
  const handleKeySubmit = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.code == "Enter") {
      locate(route)
    }
  }

  return (
    <Flex>
      <Input
        value={route}
        onChange={handleKeyChange}
        onKeyUp={handleKeySubmit}
        _focus={{ color: "gray.50" }}
        bg="gray.900"
        borderRadius="5px"
        color="gray.300"
        flex="1"
        focusBorderColor="gray.600"
        size="sm"
      />
      <Button
        _focus={{ outlineOffset: 0, border: "1px solid yellow" }}
        border="1px solid"
        borderColor="transparent"
        size="sm"
        marginLeft="10px"
        onClick={handleSubmit}
      >Go</Button>
    </Flex>
  )
}
