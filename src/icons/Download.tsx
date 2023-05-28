/**
  * Download Icon.
  *
  * @credit https://systemuicons.com/
  */
export function DownloadIcon(props: any) {
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
        transform="translate(4 3)"
      >
        <path d="m2.5 7.5 4 4.232 4-4.191" />
        <path d="m6.5.5v11" /><path d="m.5 14.5h12" />
      </g>
    </svg>
  )
}
