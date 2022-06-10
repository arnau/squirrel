import "./App.css"
import { StartScreen } from "./components/StartScreen"
import { CatalogueScreen } from "./components/CatalogueScreen"
import { useContext, useEffect } from 'react'
// import { createImage, revokeImage } from "./aux/image"
import { useWorld, Context, locate, getRoute } from "./world"
import { Value } from "./catalogue/value"

function App() {
  const ctx = useWorld("/")
  const { world, dispatch } = ctx

  useEffect(() => {
    locate(getRoute(world))
      .then((value) => {
        dispatch?.({ type: "locate", payload: value as Value })
      })
  }, [])

  return (
    <Context.Provider value={ctx}>
      <Router />
    </Context.Provider>
  )
}

function Router() {
  let { world } = useContext(Context)

  switch (world.id) {
    case "pending":
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
