import { Icon, IconButton } from "@chakra-ui/react"
import { motion } from "framer-motion"

// TODO: Clean up

const button = {
  rest: { scale: 1 },
  hover: { scale: 1.2 },
  pressed: { scale: 0.95 }
}

export function BackButton(props: any) {
  const { onClick, isDisabled } = props

  return (
    <IconButton
      colorScheme="gray"
      padding="0"
      minWidth="unset"
      height="30px"
      width="30px"
      display="block"
      disabled={isDisabled}

      aria-label="Back"
      icon={<BackIcon style={{ padding: "0px" }} />}
      onClick={onClick}
    />
  )
}

export function ForwardButton(props: any) {
  const { onClick, isDisabled } = props
  return (
    <IconButton
      colorScheme="gray"
      padding="0"
      minWidth="unset"
      height="30px"
      width="30px"
      display="block"
      disabled={isDisabled}

      aria-label="Forward"
      icon={<ForwardIcon style={{ padding: "0px" }} />}
      onClick={onClick}
    />
  )

}

/**
 * ArrowLeft Icon.
 *
 * Credit: https://systemuicons.com/
 */
export function BackIcon({ style }: any) {
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
        <g
          fill="none"
          fillRule="evenodd"
          stroke="currentColor"
          strokeLinecap="round"
          strokeLinejoin="round"
          transform="translate(3 6)"
        >
          <path d="m4.499.497-3.999 4.002 4 4.001" />
          <path d="m13.5 4.5h-13" />
        </g>
      </Icon>
    </motion.div>
  )
}

/**
 * ArrowRight Icon.
 *
 * Credit: https://systemuicons.com/
 */
export function ForwardIcon({ style }: any) {
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
        <g
          fill="none"
          fillRule="evenodd"
          stroke="currentColor"
          strokeLinecap="round"
          strokeLinejoin="round"
          transform="translate(4 6)"
        >
          <path d="m9.5.497 4 4.002-4 4.001" />
          <path d="m.5 4.5h13" />
        </g>
      </Icon>
    </motion.div>
  )
}
