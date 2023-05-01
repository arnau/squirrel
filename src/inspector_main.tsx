// @refresh reload
import { render } from 'solid-js/web'
import { Router } from "@solidjs/router"

import { InspectorScreen } from "./inspector/InspectorScreen"
import { InspectorProvider } from './inspector/InspectorContext'

render(
  () => (
    <InspectorProvider>
      <Router>
        <InspectorScreen />
      </Router>
    </InspectorProvider>
  ),
  document.getElementById('root') as HTMLElement
)
