/**
  * Minimise Icon.
  *
  * @credit https://systemuicons.com/
  */
export function MinimiseIcon(props: any) {
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
        fill="none"
        fill-rule="evenodd"
        stroke="currentColor"
        stroke-linecap="round"
        stroke-linejoin="round"
        transform="translate(3 3)"
      >
        <path d="m5.5 14.5v-5h-5" />
        <path d="m14.5 9.5h-5v5" />
        <path d="m.5 5.5h5v-5" />
        <path d="m9.5.5v5h5" />
      </g>
    </svg>
  )
}
