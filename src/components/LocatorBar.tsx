import { ReactElement, ChangeEvent } from "react"
import { Flex } from "@chakra-ui/react"
import { Input, Button } from "@chakra-ui/react"
import { invoke } from "@tauri-apps/api/tauri"
import { State } from "../state"

function submit(location: string): Promise<State> {
  return invoke("locate", { location })
}

interface LocatorBarProps {
  location: string;
  onChange: (event: ChangeEvent<HTMLInputElement>) => void;
}

export default function LocatorBar({ location, onChange }: LocatorBarProps): ReactElement {
  return (
    <Flex>
      <Input
        value={location}
        onChange={onChange}
        onKeyUp={(event) => {
          if (event.code == "Enter") {
            submit(location)
            .then((x) => console.log(x))
          }
        }}

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
      >Go</Button>
    </Flex>
  )
}
