import { Icon, IconButton } from '@chakra-ui/react'
import { ReactElement, Dispatch, SetStateAction } from "react"
import { motion } from "framer-motion"

type ExpandToggleProps = {
  setExpansion: Dispatch<SetStateAction<boolean>>;
}

const button = {
  rest: { scale: 1 },
  hover: { scale: 1.2 },
  pressed: { scale: 0.95 }
}

export function MaximiseButton({ setExpansion }: ExpandToggleProps): ReactElement {
  return (
    <IconButton
      colorScheme="gray"
      position="absolute"
      right="10px"
      top="10px"
      padding="0"
      minWidth="unset"
      height="unset"
      display="block"

      aria-label="Maximise"
      icon={<MaximiseIcon style={{ padding: "6px" }} />}
      onClick={() => setExpansion(x => !x)}
    />
  )
}

export function MinimiseButton({ setExpansion }: ExpandToggleProps): ReactElement {
  return (
    <IconButton
      position="absolute"
      right="10px"
      top="10px"
      padding="0"
      minWidth="unset"
      height="unset"
      display="block"

      aria-label="Minimise"
      icon={<MinimiseIcon style={{ padding: "6px" }} />}
      onClick={() => setExpansion(x => !x)}
    />
  )
}


/**
 * Fullscreen Icon.
 *
 * Credit: https://systemuicons.com/
 */
export function MaximiseIcon({ style }: any) {
  return (
    <motion.div
      variants={button}
      initial="rest"
      whileHover="hover"
      whileTap="pressed"
      style={style}
    >
      <Icon
        height="21"
        viewBox="0 0 21 21"
        width="21"
      >
        <g fill="none" fill-rule="evenodd" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" transform="translate(2 2)">
          <path d="m16.5 5.5v-4.978l-5.5.014" />
          <path d="m16.5.522-6 5.907" />
          <path d="m11 16.521 5.5.002-.013-5.5" />
          <path d="m16.5 16.429-6-5.907" />
          <path d="m.5 5.5v-5h5.5" />
          <path d="m6.5 6.429-6-5.907" />
          <path d="m6 16.516-5.5.007v-5.023" />
          <path d="m6.5 10.5-6 6" />
        </g>
      </Icon>
    </motion.div>
  )
}

/**
 * Minimise Icon.
 *
 * Credit: https://systemuicons.com/
 */
export function MinimiseIcon({ style }: any) {
  return (
    <motion.div
      variants={button}
      initial="rest"
      whileHover="hover"
      whileTap="pressed"
      style={style}
    >
      <Icon
        height="21"
        viewBox="0 0 21 21"
        width="21"
      >
        <g fill="none" fill-rule="evenodd" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" transform="translate(3 3)">
          <path d="m5.5 14.5v-5h-5" />
          <path d="m14.5 9.5h-5v5" />
          <path d="m.5 5.5h5v-5" />
          <path d="m9.5.5v5h5" />
        </g>
      </Icon>
    </motion.div>
  )
}
