import { extendTheme, type ThemeConfig } from '@chakra-ui/react'

const config: ThemeConfig = {
  initialColorMode: 'dark',
  useSystemColorMode: false,
  cssVarPrefix: 'sqrl',
}

const Link = {
  baseStyle: {
    display: "block",
    padding: "4px",
    color: "whitesmoke",
    _hover: {
      backgroundColor: "gray.700",
      textDecoration: "none",
    }
  },
}


const theme = extendTheme({
  config,
  colors: {
    gray: {
      300: '#B0B0B0',
      400: '#909090',
      500: '#707070',
      600: '#505050',
      700: '#303030',
      800: '#202020',
      900: '#101010',
    },
    neutral: '#505050',
  },
  components: {
    NavLink: {
      baseStyle: {
        fontSize: "small",
        ...Link.baseStyle,
      },
    },
    Link,
  }
})

export default theme
