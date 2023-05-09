import { For, Show, createEffect } from "solid-js"
import styles from "./Browser.module.scss"
import { useCatalogue } from "./CatalogueContext"
import { isRootOf, lastSegment } from "../aux/route"
import { A } from "@solidjs/router"
import { PlusIcon } from "../icons/Plus"
import { MinusIcon } from "../icons/Minus"
import { CircleIcon } from "../icons/Circle"

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
  const [{ roots }]: any = useCatalogue()

  return (
    <div class={styles.folder_tree}>
      <ul class={styles.root_list}>
        <For each={roots()}>
          {({ id, path }) => <RootItem id={id} route={path} />}
        </For>
      </ul>
    </div>
  )
}

function FolderDetails() {
  const [{ state }, { toggleFolderDetails }]: any = useCatalogue()
  const handleClick = () => {
    toggleFolderDetails()
  }
  return (
    <div class={styles.folder_details}>
      <h1 onClick={handleClick}>Folder details</h1>
    </div>
  )
}

function RootItem(props: any) {
  const [{ route, tree }]: any = useCatalogue()
  const isOpen = () =>
    tree.kind === "Node" && props.route === tree.path && isRootOf(route(), props.route)

  return (
    <li id={props.id}>
      <RootAction id={props.id} route={props.route} isOpen={isOpen} />
      <Show when={isOpen()}>
        <ul>
          <For each={tree.children}>
            {(child) => <TreeItem value={child} />}
          </For>
        </ul>
      </Show>
    </li>
  )
}


function RootAction(props: any) {
  const [{ route, state }, { toggleRoot, toggleTreeNode }]: any = useCatalogue()
  const isOpen = () => !!state.tree[props.route]?.isOpen
  const isSelected = () => route() === props.route
  const wrapClassList = () => ({
    [styles.selected]: isSelected(),
    [styles.tree_action]: true,
  })

  const handleClick = (event: MouseEvent) => {
    const target = event.target as HTMLElement

    if (target.nodeName == "A") {
      toggleRoot(props.route)
    }
  }



  return (
    <span classList={wrapClassList()}>
      {
        props.isOpen()
          ? <span class={styles.noop}><MinusIcon /></span>
          : <span class={styles.noop}><PlusIcon /></span>
      }
      <A
        onClick={handleClick}
        href={`/catalogue/${props.route}`}>{props.route}</A>
    </span>
  )
}

function TreeItem(props: any) {
  return (
    <li>
      {
        props.value.kind === "Node"
          ? <TreeNode route={props.value.path} children={props.value.children} />
          : <TreeLeaf route={props.value.path} />
      }
    </li>
  )
}

function TreeNode(props: any) {
  const [{ state }]: any = useCatalogue()
  const isOpen = () => !!state.tree[props.route]?.isOpen

  return (
    <>
      <TreeAction route={props.route} kind="Node" isOpen={isOpen} />
      <Show when={props.children.length > 0 && isOpen()}>
        <ul>
          <For each={props.children}>
            {(child) => <TreeItem value={child} />}
          </For>
        </ul>
      </Show>
    </>
  )
}

function TreeLeaf(props: any) {
  return (
    <TreeAction route={props.route} />
  )
}

function TreeAction(props: any) {
  const [{ route }, { toggleTreeNode }]: any = useCatalogue()
  const text = () => lastSegment(props.route)
  const isSelected = () => route() === props.route
  const wrapClassList = () => ({
    [styles.selected]: isSelected(),
    [styles.tree_action]: true,
  })

  const handleToggle = () => {
    toggleTreeNode(props.route)
  }


  return (
    <span classList={wrapClassList()}>
      <Show
        when={props.kind === "Node"}
        fallback={<span class={styles.noop}><CircleIcon /></span>}>
        {
          props.isOpen()
            ? <CloseButton onClick={handleToggle} />
            : <OpenButton onClick={handleToggle} />
        }
      </Show>
      <A
        title={text()!}
        href={`/catalogue/${props.route}`}>{text()}</A>
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
