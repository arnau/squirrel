/**
  * Flag Icon.
  *
  * @credit https://systemuicons.com/
  */
export function FlagIcon(props: any) {
  const height = () =>
    props.size
  const width = () =>
    props.size

  return (
    <svg
      viewBox="0 0 21 21"
      height={height()}
      width={width()}
      xmlns="http://www.w3.org/2000/svg"
    >

      <g
        fill={props.fill ? "currentColor" : "none"}
        fill-rule="evenodd"
        stroke="currentColor"
        stroke-linecap="round"
        stroke-linejoin="round"
        transform="translate(5 4)"
      >
      <path d="m.5 13.5v-11" />
      <path d="m.5 2.5c.66666667-1.33333333 1.66666667-2 3-2 2 0 2 2 4 2 1.33333333 0 2.33333333-.33333333 3-1v6c-.66666667.66666667-1.66666667 1-3 1-2 0-2-2-4-2-1.33333333 0-2.33333333.66666667-3 2z" />
      </g>
    </svg>
  )
}
