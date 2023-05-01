
import { For, Show } from "solid-js"
import styles from "./Inspector.module.scss"
import { useInspector } from "./InspectorContext"

export function Table() {
  const [{ log }]: any = useInspector()

  return (
    <table class={styles.log}>
      <caption>Total entries {log().length}</caption>
      <thead>
        <tr>
          <th>stamp</th>
          <th>action</th>
          <th>bag</th>
        </tr>
      </thead>
      <tbody>
        <For each={log()}>
          {
            (entry, i) => <Row index={i()} {...entry} />
          }
        </For>
      </tbody>
    </table>
  )
}


function Row(props: any) {
  return (
    <tr>
      <td>{props.stamp.substring(0, props.stamp.indexOf('.'))}</td>
      <td>{props.action}</td>
      <td><Bag value={props.bag} /></td>
    </tr>
  )
}


function Bag(props: any) {
  return (
    <Show when={props.value !== null && props.value !== undefined}>
      <ul class={styles.bag}>
        <For each={Object.entries(props.value)}>
          {
            (item: [any, any]) => <BagItem key={item[0]} value={item[1]} />
          }
        </For>
      </ul>
    </Show>
  )
}

function BagItem(props: any) {
  return (
    <li>{props.key}: {props.value}</li>
  )
}
