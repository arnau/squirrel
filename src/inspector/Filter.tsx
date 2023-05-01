import { useInspector } from "./InspectorContext"
import styles from "./Inspector.module.scss"

export function Filter() {
  const [
    { filter, deleteToggle },
    { setFilter, revertFilter, submitFilter, deleteFiltered, toggleDelete }
  ]: any = useInspector()

  const handleSubmit = (event: SubmitEvent) => {
    event.preventDefault()
    event.stopPropagation()

    submitFilter()
  }

  const handleInput = (event: InputEvent) => {
    event.preventDefault()
    event.stopPropagation()

    setFilter((event.currentTarget as HTMLTextAreaElement).value)
  }

  const handleEnterSubmit = (event: any) => {
    if (event.keyCode == 13 && event.metaKey) {
      event.preventDefault()
      event.stopPropagation()

      submitFilter()
    }
  }

  const handleRevert = (event: any) => {
    event.preventDefault()
    event.stopPropagation()

    revertFilter()
  }

  const handleDelete = (event: any) => {
    event.preventDefault()
    event.stopPropagation()

    if (deleteToggle()) {
      deleteFiltered()
    } else {
      toggleDelete()
    }
  }



  return (
    <form id="inspector_filter" class={styles.filter} onSubmit={handleSubmit}>
      <textarea id="inspector_filter_box"
        onInput={handleInput}
        value={filter()}
        autofocus={true}
        onKeyDown={handleEnterSubmit}
      />

      <div class={styles.button_group}>
        <button type="submit">Filter</button>
        <button type="button" onClick={handleRevert}>Revert</button>
        <button type="button" classList={{ [styles.dangerButton]: deleteToggle() }} onClick={handleDelete}>Delete</button>
      </div>
    </form>
  )
}
