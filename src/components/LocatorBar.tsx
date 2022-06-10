import { ReactElement, ChangeEvent, KeyboardEvent, MouseEvent, useState, useContext } from "react"
import { Flex } from "@chakra-ui/react"
import { Input, Button } from "@chakra-ui/react"
import { Context, getRoute, locate } from "../world"
import { Value } from "../catalogue/value"


export default function LocatorBar(): ReactElement {
  const { world, dispatch } = useContext(Context)
  const route = getRoute(world)
  const [newRoute, setRoute] = useState(route)
  const submit = () => {
    locate(newRoute)
      .then((value) => {
        console.log(value)
        dispatch?.({ type: "locate", payload: value as Value })
      })
      .catch((err) => {
        console.error(err)
      })
  }
  const handleKeyChange =
    (event: ChangeEvent<HTMLInputElement>) => setRoute(event.target.value)
  const handleKeySubmit = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.code == "Enter") {
      submit()
    }
  }
  const handleSubmit = (event: MouseEvent<HTMLButtonElement>) => {
    console.log(event)
    submit()
  }

  return (
    <Flex>
      <Input
        value={newRoute}
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
