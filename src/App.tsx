// import { createImage, revokeImage } from "./aux/image"
import "./App.css"
import { StartScreen } from "./components/StartScreen"
import { CatalogueScreen } from "./components/CatalogueScreen"
import { useStore } from "./world"
import { useEffect } from "react"

function App() {
  const locate = useStore(state => state.locate)
  const currentRoute = useStore(state => state.getCurrentRoute())

  useEffect(() => {
    // Initial fetch.
    locate(currentRoute)
  }, [])


  return (
    <Router />
  )
}

function Router() {
  let worldId = useStore(state => state.world.id)

  switch (worldId) {
    case "void":
      return <StartScreen />
    case "catalogue":
      return <CatalogueScreen />
    default:
      throw new Error("Unknown world id")
  }
}

// function AppBack() {
//   const [blob, setBlob] = useState<string | null>(null)
//
//   return (
//     <div className="App">
//       <header className="App-header">
//         <p>
//           <button type="button" onClick={
//             () => invoke('state').then(raw => setBlob(createImage(raw as number[], 'image/png')))
//           }>
//             state
//           </button>
//           <button type="button" onClick={() => {
//             revokeImage(blob);
//             setBlob(null);
//           }}>
//             revoke
//           </button>
//         </p>
//       </header>
//       <main>
//         {
//           (blob)
//             ? <img src={blob} width="500" />
//             : <span>noop</span>
//         }
//       </main>
//     </div>
//   )
// }


export default App
