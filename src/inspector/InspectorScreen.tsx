import { Table } from "./Table"
import styles from "./Inspector.module.scss"
import { Filter } from "./Filter"


export function InspectorScreen() {
  return (
    <div class={styles.wrap}>
      <Filter />
      <Table />
    </div>
  )
}
