import { For, Show } from "solid-js"
import { useConnectorsSection } from "./ConnectorsSectionContext"
import styles from "./Preferences.module.scss"

export function ConnectorsSection() {
  return (
    <>
      <h1>Connectors</h1>
      <Form />
      <List />
    </>
  )
}

function Form() {
  return (
    <section class={styles.downloads_section}>
      <h2>New connector</h2>
      <p>Log in to BackBlaze,
        go to Application Keys,
        create a new key with
        access to a single bucket (do not use "All")
        Read Only.
      </p>
      <div class={styles.group}>
        <label id="keyname_label" aria-control="keyname">Key Name</label>
        <input
          id="keyname"
          type="text"
          aria-labelledby="keyname_label"
          value={""}
        />
      </div>
      <div class={styles.group}>
        <label id="bucketname_label" aria-control="bucketname">Bucket Name</label>
        <input
          id="bucketname"
          type="text"
          aria-labelledby="bucketname_label"
          value={""}
        />
      </div>
      <div class={styles.group}>
        <label id="key_label" aria-control="key">Application Key</label>
        <input
          id="key"
          type="text"
          aria-labelledby="key_label"
          value={""}
        />
      </div>

      <div class={styles.action_group}>
        <button
          id="cancel"
          onClick={() => { }}
        >Cancel</button>
        <button
          id="save"
          onClick={() => { }}
        >Save</button>
      </div>

    </section>
  )
}

function List() {
  const [{ connectorsList }]: any = useConnectorsSection()

  return (
    <Show when={connectorsList().length > 0}>
      <section class={styles.downloads_section}>
        <ul>
          <For each={connectorsList()}>
            {(data) => <Connector {...data} />}
          </For>
        </ul>
      </section>
    </Show>
  )
}

function Connector(props: any) {
  return (
    <li id={props.id}>
      <span>Name: {props.name}</span> <button>Remove</button>
    </li>
  )
}
