import { Show, createEffect, createSignal } from "solid-js"
import styles from "./Browser.module.scss"
import { useCatalogue } from "./CatalogueContext"
import { convertFileSrc } from "@tauri-apps/api/tauri"
import { Asset } from "./types"
import { FullscreenIcon } from "../icons/Fullscreen"
import { MinimiseIcon } from "../icons/Minimise"
import { DownloadIcon } from "../icons/Download"

export function ImagePane() {
  const [{ location, assetStore, state }]: any = useCatalogue()
  const id = () =>
    location().id
  const asset = () =>
    assetStore.assets?.find((asset: Asset) => asset.id === location()?.id)
  const isAsset = () =>
    location()?.kind === "Asset" && asset()
  const isFullsize = () =>
    state.isImageFullsize

  const classList = () => ({
    [styles.image_pane]: true,
    [styles.selected]: isAsset(),
  })

  return (
    <div classList={classList()}>
      <Show when={isAsset()}>
        <div class={styles.toolbar}>
          <ToggleSizeButton />
          <DownloadButton />
        </div>

        <Canvas trackId={id()} fullsize={isFullsize()}>
          <Image id={id()} asset={asset()} />
        </Canvas>
      </Show>
    </div>
  )
}

interface Pos {
  left: number,
  top: number,
  x: number,
  y: number,
}

function Canvas(props: any) {
  const [, { toggleImageSize }]: any = useCatalogue()
  let el: HTMLDivElement | undefined
  const [pos, setPos] = createSignal<Pos | undefined>()
  const classList = () => ({
    [styles.canvas]: true,
    [styles.fullsize]: props.fullsize,
  })

  createEffect(() => {
    if (el !== undefined) {
      el.scrollLeft = 0
      el.scrollTop = 0
    }
  })

  const startHandler = (event: MouseEvent) => {
    if (!props.fullsize) {
      return
    }

    el!.style.cursor = "grabbing";
    el!.style.userSelect = "none";

    event.stopPropagation()
    event.preventDefault()

    setPos({
      left: el!.scrollLeft,
      top: el!.scrollTop,
      x: event.clientX,
      y: event.clientY,
    })
  }

  const dragHandler = (event: MouseEvent) => {
    event.stopPropagation()
    event.preventDefault()

    if (pos() !== undefined) {
      setPos(pos => {
        const dx = event.clientX - pos!.x;
        const dy = event.clientY - pos!.y;

        return ({
          left: pos!.left - dx,
          top: pos!.top - dy,
          x: event.clientX,
          y: event.clientY,
        })
      })

      el!.scrollLeft = pos()!.left
      el!.scrollTop = pos()!.top
    }
  }

  const stopHandler = (event: MouseEvent) => {
    event.stopPropagation()
    event.preventDefault()

    el!.style.removeProperty("cursor")
    el!.style.removeProperty("user-select")

    setPos(undefined)
  }

  const toggleHandler = (event: MouseEvent) => {
    event.stopPropagation()
    event.preventDefault()

    toggleImageSize()
  }

  return (
    <div
      ref={el}
      classList={classList()}
      onMouseDown={startHandler}
      onMouseMove={dragHandler}
      onMouseUp={stopHandler}
      onMouseLeave={stopHandler}
      onDblClick={toggleHandler}
    >
      {props.children}
    </div>
  )
}

function Image(props: any) {
  const url = () =>
    convertFileSrc(`${props.id}.max`, "image")
  const width = () =>
    props.asset.metadata.width
  const height = () =>
    props.asset.metadata.height

  return (
    <img
      src={url()}
      alt=""
      height={height()}
      width={width()}
    />
  )
}


function ToggleSizeButton() {
  const [{ state }, { toggleImageSize }]: any = useCatalogue()
  const size = "24"
  const handleClick = () => {
    toggleImageSize()
  }

  return (
    <button
      type="button"
      onClick={handleClick}
      aria-label="maximise"
      title="toggle size"
      class={styles.maximise_button}
    >
      <Show when={state.isImageFullsize} fallback={<FullscreenIcon size={size} />}>
        <MinimiseIcon size={size} />
      </Show>
    </button>
  )
}

function DownloadButton() {
  const size = "24"
  const handleClick = () => {
    console.log("unimplemented")
  }

  return (
    <button
      type="button"
      onClick={handleClick}
      aria-label="maximise"
      title="download"
      class={styles.download_button}
    >
      <DownloadIcon size={size} />
    </button>
  )
}
