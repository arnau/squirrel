import styles from "./Browser.module.scss"
import { A } from "@solidjs/router"
import { CircleIcon } from "../icons/Circle"
import { For, Show } from "solid-js"
import { MinusIcon } from "../icons/Minus"
import { PlusIcon } from "../icons/Plus"
import { lastSegment } from "../aux/route"
import { useCatalogue } from "./CatalogueContext"

export function TreePane() {
  const [{ state }]: any = useCatalogue()

  const paneClassList = () => ({
    [styles.tree_pane]: true,
    [styles.folder_details_open]: state.isDetailsOpen,
    [styles.folder_details_close]: !state.isDetailsOpen,
  })

  return (
    <div classList={paneClassList()}>
      <FolderTree />
      <FolderDetails />
    </div>
  )
}

function FolderTree() {
  const [{ ground }]: any = useCatalogue()

  return (
    <div class={styles.folder_tree}>
      <ul class={styles.root_list}>
        <For each={ground()}>
          {data => <Folder {...data} />}
        </For>
      </ul>
    </div>
  )
}

function Folder(props: any) {
  const [{ state, folderMap }]: any = useCatalogue()
  const children = () =>
    folderMap[props.id] ?? []
  const isOpen = () =>
    !!state.tree[props.id]?.isOpen && children().length > 0
  const isNode = () =>
    props.counter > 0

  return (
    <li id={props.id}>
      <Action id={props.id} path={props.path} isNode={isNode} isOpen={isOpen} />
      <Show when={isNode() && isOpen()}>
        <ul>
          <For each={children()}>
            {data => <Folder {...data} />}
          </For>
        </ul>
      </Show>
    </li>
  )
}

function Action(props: any) {
  const [{ location }, { toggleTreeNode }]: any = useCatalogue()
  const text = () =>
    lastSegment(props.path)
  const trail = () =>
    location().trail ?? []
  const isSelected = () =>
    props.id === location()?.id || (location().kind === "Asset" && props.id === trail()[trail().length - 1])
  const wrapClassList = () => ({
    [styles.selected]: isSelected(),
    [styles.tree_action]: true,
  })

  const handleToggle = () => {
    toggleTreeNode(props.id)
  }

  return (
    <span classList={wrapClassList()}>
      <Show
        when={props.isNode()}
        fallback={<span class={styles.noop}><CircleIcon /></span>}>
        {
          props.isOpen()
            ? <CloseButton onClick={handleToggle} />
            : <OpenButton onClick={handleToggle} />
        }
      </Show>
      <A
        title={text()!}
        href={`/catalogue/${props.id}`}>{text()}</A>
    </span>
  )
}

function OpenButton(props: any) {
  return (
    <button
      type="button"
      onClick={props.onClick}
      aria-label="open"
    >
      <PlusIcon />
    </button>
  )
}

function CloseButton(props: any) {
  return (
    <button
      type="button"
      onClick={props.onClick}
      aria-label="close"
    >
      <MinusIcon />
    </button>
  )
}

function FolderDetails() {
  const [{ folderDetails, state }, { toggleFolderDetails }]: any = useCatalogue()
  const handleClick = () => {
    toggleFolderDetails()
  }
  return (
    <div class={styles.folder_details}>
      <h1 onClick={handleClick}>Folder details</h1>

      <Show when={state.isDetailsOpen && folderDetails() !== undefined}>
        <ul>
          <li>subfolders: {folderDetails().folder_count}</li>
          <li>assets: {folderDetails().asset_count}</li>

          <li>root path: {folderDetails().root.path}</li>

          <li>source name: {folderDetails().source.name}</li>
          <li>source path: {folderDetails().source.path}</li>
          <li>source version: {folderDetails().source.version}</li>
        </ul>
      </Show>
    </div>
  )
}
