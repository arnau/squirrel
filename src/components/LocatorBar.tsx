import { ReactElement, ChangeEvent, KeyboardEvent, MouseEvent } from "react"
import { Flex, Icon, IconButton } from "@chakra-ui/react"
import { Input, Button } from "@chakra-ui/react"
import { motion } from "framer-motion"
import { useStore } from "../world"
import { Route } from "../aux/route"
import { BackButton, ForwardButton } from "./LocatorButtonBar"


export default function LocatorBar(): ReactElement {
  const locate = useStore(state => state.locate)
  const setRoute = useStore(state => state.setRoute)
  const route = useStore(state => state.getRoute()) as Route

  // history
  const add = useStore(state => state.add)
  const back = useStore(state => state.back)
  const forward = useStore(state => state.forward)
  const isFirst = useStore(state => state.isFirst)
  const isLast = useStore(state => state.isLast)
  const getCurrentRoute = useStore(state => state.getCurrentRoute)
  const getHistory = useStore(state => state.getHistory)

  const locate_ = async (route: Route) => {
    if (getCurrentRoute() !== route) {
      // TODO if locate is error, display error and go back.
      locate(route)
      console.log(getHistory())
    }
  }

  // handlers
  const handleSubmit = (_event: MouseEvent<HTMLButtonElement>) =>
    locate(route)

  const handleKeyChange = (event: ChangeEvent<HTMLInputElement>) =>
    setRoute(event.target.value)

  const handleKeySubmit = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.code == "Enter") {
      locate(route)
    }
  }

  const handleBack = () => {
    const currentRoute = getCurrentRoute()
    const newRoute = back()

    if (newRoute !== currentRoute) {
      locate(newRoute)
    }
  }

  const handleForward = () => {
    const currentRoute = getCurrentRoute()
    const newRoute = forward()

    if (newRoute !== currentRoute) {
      locate(newRoute)
    }
  }

  return (
    <Flex gap={2}>
      <BackButton onClick={handleBack} isDisabled={isFirst()} />
      <ForwardButton onClick={handleForward} isDisabled={isLast()} />

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
        onClick={handleSubmit}
      >Go</Button>
    </Flex>
  )
}
