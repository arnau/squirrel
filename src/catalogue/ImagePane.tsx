import { Show, createEffect } from "solid-js"
import styles from "./Browser.module.scss"
import { useCatalogue } from "./CatalogueContext"
import { convertFileSrc } from "@tauri-apps/api/tauri"
import { Asset } from "./types"
import { FullscreenIcon } from "../icons/Fullscreen"

export function ImagePane() {
  const [{ location, assetStore }]: any = useCatalogue()
  const id = () =>
    location().id
  const asset = () =>
    assetStore.assets?.find((asset: Asset) => asset.id === location()?.id)
  const isAsset = () =>
    location()?.kind === "Asset" && asset()

  const classList = () => ({
    [styles.image_pane]: true,
    [styles.selected]: isAsset(),
  })

  return (
    <div classList={classList()}>
      <Show when={isAsset()}>
        <Image id={id()} asset={asset()} />
      </Show>
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
    <>
      <MaximiseButton />
      <img
        src={url()}
        alt=""
        height={height()}
        width={width()}
      />
    </>
  )
}

function MaximiseButton() {
  const handleClick = () => { }

  return (
    <button
      type="button"
      onClick={handleClick}
      aria-label="maximise"
      title="maximise"
      class={styles.maximise_button}
    >
      <FullscreenIcon size="24" />
    </button>
  )
}
