/**
  * Star Icon.
  *
  * @credit https://systemuicons.com/
  */
export function StarIcon(props: any) {
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
      <path
        d="m7.5 11.5-5 3 2-5.131-4-3.869h5l2-5 2 5h5l-4 4 2 5z"
        fill={props.fill ? "currentColor" : "none"}
        stroke="currentColor"
        stroke-linecap="round"
        stroke-linejoin="round"
        transform="translate(3 3)"
      />
    </svg>
  )
}
