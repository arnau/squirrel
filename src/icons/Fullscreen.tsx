/**
  * Fullscreen icon.
  *
  * @credit https://systemuicons.com/
  */
export function FullscreenIcon(props: any) {
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
        transform="translate(2 2)"
      >
        <path d="m16.5 5.5v-4.978l-5.5.014" />
        <path d="m16.5.522-6 5.907" />
        <path d="m11 16.521 5.5.002-.013-5.5" />
        <path d="m16.5 16.429-6-5.907" />
        <path d="m.5 5.5v-5h5.5" />
        <path d="m6.5 6.429-6-5.907" />
        <path d="m6 16.516-5.5.007v-5.023" />
        <path d="m6.5 10.5-6 6" />
      </g>
    </svg>
  )
}
