import { For, Show, createEffect } from "solid-js"
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

function FormGroup(props: any) {
  const [{ form }, { setForm, cancelForm, submitForm }]: any = useConnectorsSection()
  const id = () => props.id
  const label = () => props.label
  const label_id = () => `${id()}_label`

  let ref: HTMLInputElement | undefined

  return (
      <div class={styles.group}>
        <label id={label_id()} aria-control={id()}><span>{label()}</span></label>
        <input
          ref={ref}
          id={id()}
          type="text"
          aria-labelledby={label_id()}
          autocomplete="off"
          autocorrect="off"
          onBlur={() => setForm(id(), ref!.value)}
          value={form[id()]}
        />
      </div>
  )
}

function Form() {
  const [{ formError }, { cancelForm, submitForm }]: any = useConnectorsSection()

  return (
    <section class={styles.new_connector_section}>
      <h2>New connector</h2>
      <p>
        Log in to BackBlaze,
        go to Application Keys,
        create a new key with access to a single bucket (do not use "All")
        and make sure it's Read Only.
      </p>

      <Show when={formError()}>
        <div class={styles.form_error}>
          {formError()}
        </div>
      </Show>

      <FormGroup id="id" label="Key Id" />
      <FormGroup id="key_name" label="Key Name" />
      <FormGroup id="bucket_name" label="Bucket Name" />
      <FormGroup id="secret_key" label="Application Key" />

      <div class={styles.action_group}>
        <button
          id="cancel"
          onClick={cancelForm}
        >Cancel</button>
        <button
          id="save"
          onClick={submitForm}
        >Save</button>
      </div>
    </section>
  )
}

function List() {
  const [{ connectorsList }]: any = useConnectorsSection()

  return (
    <Show when={connectorsList().length > 0}>
      <section class={styles.connectors_section}>
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
  const [actionErrors, { removeConnector }]: any = useConnectorsSection()
  const error = () => actionErrors[props.id]

  return (
    <li id={props.id} class={styles.connector_item}>
      <Show when={error()}>
        <div class={styles.form_error}>
          {error()}
        </div>
      </Show>

      <ConnectorGroup key="Creation stamp" value={props.creation_stamp} />
      <ConnectorGroup key="Key Id" value={props.id} />
      <ConnectorGroup key="Key Name" value={props.key_name} />
      <ConnectorGroup key="Bucket Name" value={props.bucket_name} />

      <div class={styles.action_group}>
        <button
          id={`remove-${props.id}`}
          onClick={() => removeConnector(props.id)}
        >Remove</button>
      </div>
    </li>
  )
}

function ConnectorGroup(props: any) {
  return (
    <div class={styles.group}>
      <span class={styles.key}>{props.key}</span>
      <span class={styles.value}>{props.value}</span>
    </div>
  )
}
