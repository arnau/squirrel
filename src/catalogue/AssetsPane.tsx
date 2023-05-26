import { For, Show, children, createEffect, createSignal } from "solid-js"
import styles from "./Browser.module.scss"
import { useCatalogue } from "./CatalogueContext"
import { A } from "@solidjs/router"
import { lastSegment } from "../aux/route"
import { createVisibilityObserver } from "@solid-primitives/intersection-observer"
import type { Thumbnail } from "./types"

export function AssetsPane() {
  const [{ assetStore }]: any = useCatalogue()
  const isEmpty = () =>
    (assetStore.assets ?? []).length === 0
  const classList = () => ({
    [styles.empty]: isEmpty(),
    [styles.assets_pane]: true,
  })


  return (
    <div classList={classList()}>
      <Show when={!isEmpty()} fallback={<span>(no assets)</span>}>
        <For each={assetStore.assets}>
          {data => <Asset {...data} />}
        </For>
      </Show>

    </div>
  )
}

//   // TODO: Instead of effect it should run after a folder click event.
//   // const elementRef = useRef<HTMLDivElement | null | undefined>()
//   // useEffect(() => {
//   //   if (elementRef !== null && elementRef !== undefined) {
//   //     elementRef.current!.scrollTo({
//   //       top: 0,
//   //       behavior: 'smooth',
//   //     })
//   //   }
//   // }, [])



function Asset(props: any) {
  const [{ location }]: any = useCatalogue()
  const isSelected = () =>
    props.id === location()?.id
  const isVirtual = () =>
    !!props.master_id
  const route = () =>
    `/catalogue/${props.id}`
  const title = () =>
    props.path
  const text = () =>
    lastSegment(props.path)!.split("#")[0]
  const metadata = () =>
    props.metadata


  let el: HTMLAnchorElement | undefined;
  const useVisibilityObserver = createVisibilityObserver({ threshold: 0.2 })
  const isVisible = useVisibilityObserver(() => el)

  const classList = () => ({
    [styles.selected]: isSelected(),
    [styles.asset]: true,
    [styles.loading]: !isVisible(),
  })

  return (
    <A
      ref={el}
      id={props.id}
      classList={classList()}
      title={title()}
      href={route()}
    >

      <Show when={isVisible()} fallback={<div>Loading...</div>}>
        <Header isVirtual={isVirtual}>{text}</Header>
        <Thumbnail id={props.id} isVisible={isVisible} />

        <Metapoint label="Format">{metadata().format}</Metapoint>
        <Metapoint label="Stars">{metadata().rating}</Metapoint>
        <Metapoint label="Flagged">{metadata().flag ? "Yes" : "No"}</Metapoint>
        <Metapoint label="Colour">{metadata().label}</Metapoint>
        <Metapoint label="Size" className="last">{`${metadata().width} x ${metadata().height}`}</Metapoint>
      </Show>
    </A>
  )
}


function Header(props: any) {
  const value = children(() => props.children);

  return (
    <div class={styles.header}>
      <Show when={props.isVirtual()} fallback={value()}>
        <span class={styles.badge}>virtual</span> {value()}
      </Show>
    </div>
  )
}

function Thumbnail(props: any) {
  const [, { fetchThumbnail }]: any = useCatalogue()
  const [thumbnail, setThumbnail] = createSignal<Thumbnail>()
  const url = () => dataUrl(thumbnail())

  createEffect(async () => {
    if (props.isVisible() && thumbnail() === undefined) {
      setThumbnail(await fetchThumbnail(props.id))
    }
  })

  const img =
    <img
      src={url()}
      // blocks the UI. image_protocol should be async
      // src={`image://localhost/${id}.thumb`}
      alt=""
    />

  return (
    <div class={styles.thumbnail}>
      {img}
    </div>
  )
}

function Metapoint(props: any) {
  return (
    <>
      <div class={styles.label}>{props.label}</div>
      <div class={styles.datum}>{props.children}</div>
    </>
  )
}

function dataUrl(data: Thumbnail | undefined): string {
  return data === undefined
    ? ''
    : `data:image/jpeg;base64,${data}`
}
