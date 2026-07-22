import nextra from 'nextra'

const withNextra = nextra({})

const basePath =
  process.env.NODE_ENV === 'production' ? '/rust-for-kotlin-devs' : ''

export default withNextra({
  output: 'export',
  images: { unoptimized: true },
  basePath,
  trailingSlash: true,
  env: { NEXT_PUBLIC_BASE_PATH: basePath },
})
