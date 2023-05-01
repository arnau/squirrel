import { invoke } from '@tauri-apps/api';
import { createContext, createMemo, createSignal, onMount, useContext } from 'solid-js';


type EventLog = Array<any>

export const InspectorContext = createContext()

export function InspectorProvider(props: any) {
  const [log, setLog] = createSignal<EventLog>([])
  const [filter, setFilter] = createSignal("")
  const [currentFilter, setCurrentFilter] = createSignal("")
  const [deleteToggle, setDeleteToggle] = createSignal(false)

  const fetch = async () => {
    const res: EventLog = await invoke("inspect_logs", { query: filter() })

    setLog(res)
    setCurrentFilter(filter())
  }
  const prune = async () => {
    const res: EventLog = await invoke("prune_logs", { query: filter() })

    setLog(res)
    setCurrentFilter(filter())
  }

  onMount(async () => {
    await fetch()
  })


  const value = [
    // read
    {
      log,
      filter,
      deleteToggle,
    },

    // write
    {
      setFilter,
      addFilter(text: string) {
        setDeleteToggle(false)
        setFilter(text)
      },
      cleanFilter() {
        setDeleteToggle(false)
        setFilter("")
      },
      revertFilter() {
        setDeleteToggle(false)
        setFilter(currentFilter())
      },
      submitFilter() {
        setDeleteToggle(false)

        fetch()
          .catch((e) => {
            console.error(e)
          })
      },
      deleteFiltered() {
        setDeleteToggle(false)

        prune()
          .catch((e) => {
            console.error(e)
          })
      },
      toggleDelete() {
        setDeleteToggle(!deleteToggle())
      },
    }
  ]

  return (
    <InspectorContext.Provider value={value} >
      {props.children}
    </InspectorContext.Provider>
  )
}

export function useInspector() { return useContext(InspectorContext); }
