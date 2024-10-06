// TODO: DEV-only
const basePath = process.env.__NEXT_ROUTER_BASEPATH || ''
const pathname = `${basePath}/__nextjs_source-map`

export function findSourceMapURL(filename: string): string | null {
  const url = new URL(pathname, document.location.origin)

  url.searchParams.set('filename', filename)

  return url.href
}
