/** Transforms a Rust `Vec<u8>` image to an object URL.
  *
  * TODO: This function creates an object URL but needs to be released with `URL.revokeObjectURL`. See: https://developer.mozilla.org/en-US/docs/Web/API/URL/createObjectURL
  */
export function createImage(raw: Array<number>, mimetype: string): string {
  const bytes = new Uint8Array(raw)
  const blob = new Blob([bytes.buffer], {type: mimetype})

  return URL.createObjectURL(blob)
}

export function revokeImage(url: string | null) {
  if (url) {
    URL.revokeObjectURL(url)
  }
}
