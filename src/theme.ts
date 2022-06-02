import { extendTheme, type ThemeConfig } from '@chakra-ui/react'

const config: ThemeConfig = {
  initialColorMode: 'dark',
  useSystemColorMode: false,
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
  }
})

export default theme
